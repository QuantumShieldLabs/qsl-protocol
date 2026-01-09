#!/usr/bin/env python3
from __future__ import annotations

import argparse, json, zipfile

def find_openapi_member(z: zipfile.ZipFile) -> str:
    preferred = [n for n in z.namelist()
                 if not n.endswith("/")
                 and ("p3-13" in n.lower() or "p3_13" in n.lower())
                 and (n.lower().endswith(".yaml") or n.lower().endswith(".yml"))
                 and ("openapi" in n.lower())]
    if preferred:
        return sorted(preferred)[0]
    yamls = [n for n in z.namelist()
             if not n.endswith("/")
             and (n.lower().endswith(".yaml") or n.lower().endswith(".yml"))]
    if not yamls:
        raise RuntimeError("No YAML files found in Phase3 zip to validate as OpenAPI.")
    return sorted(yamls)[0]

def is_extension_key(key: str) -> bool:
    return key.startswith("x-")

def add_error(report: dict, msg: str) -> None:
    report["errors"].append(msg)

def check_allowed_keys(report: dict, obj: dict, allowed: set[str], ctx: str) -> None:
    for key in obj:
        if key not in allowed and not is_extension_key(key):
            add_error(report, f"{ctx}: unexpected key '{key}'")

def validate_info(report: dict, info: dict) -> None:
    if not isinstance(info, dict):
        add_error(report, "info: must be an object")
        return
    check_allowed_keys(report, info, {"title", "version", "description", "termsOfService", "contact", "license"}, "info")
    if not isinstance(info.get("title"), str):
        add_error(report, "info.title: must be string")
    if not isinstance(info.get("version"), str):
        add_error(report, "info.version: must be string")
    if "description" in info and not isinstance(info["description"], str):
        add_error(report, "info.description: must be string")

def validate_tags(report: dict, tags: list) -> None:
    if not isinstance(tags, list):
        add_error(report, "tags: must be array")
        return
    for idx, tag in enumerate(tags):
        if not isinstance(tag, dict):
            add_error(report, f"tags[{idx}]: must be object")
            continue
        check_allowed_keys(report, tag, {"name", "description", "externalDocs"}, f"tags[{idx}]")
        if not isinstance(tag.get("name"), str):
            add_error(report, f"tags[{idx}].name: must be string")
        if "description" in tag and not isinstance(tag["description"], str):
            add_error(report, f"tags[{idx}].description: must be string")

def validate_security(report: dict, security: list) -> None:
    if not isinstance(security, list):
        add_error(report, "security: must be array")
        return
    for idx, sec in enumerate(security):
        if not isinstance(sec, dict):
            add_error(report, f"security[{idx}]: must be object")
            continue
        for name, scopes in sec.items():
            if not isinstance(name, str):
                add_error(report, f"security[{idx}] key: must be string")
            if not isinstance(scopes, list) or not all(isinstance(s, str) for s in scopes):
                add_error(report, f"security[{idx}].{name}: must be array of strings")

def build_components_index(components: dict) -> dict[str, set[str]]:
    idx: dict[str, set[str]] = {}
    for group in ("schemas", "responses", "parameters", "securitySchemes"):
        items = components.get(group, {})
        if isinstance(items, dict):
            idx[group] = set(items.keys())
        else:
            idx[group] = set()
    return idx

def ref_exists(ref: str, idx: dict[str, set[str]]) -> bool:
    if not ref.startswith("#/components/"):
        return False
    parts = ref.split("/")
    if len(parts) != 4:
        return False
    _, _, group, name = parts
    return group in idx and name in idx[group]

def validate_schema_objects(report: dict, schemas: dict) -> None:
    from jsonschema import Draft202012Validator
    if not isinstance(schemas, dict):
        add_error(report, "components.schemas: must be object")
        return
    for name, schema in schemas.items():
        if not isinstance(schema, dict):
            add_error(report, f"components.schemas.{name}: must be object")
            continue
        try:
            Draft202012Validator.check_schema(schema)
        except Exception as e:
            add_error(report, f"components.schemas.{name}: invalid JSON Schema: {e}")

def validate_parameter_obj(report: dict, param: dict, ctx: str) -> None:
    if not isinstance(param, dict):
        add_error(report, f"{ctx}: must be object")
        return
    check_allowed_keys(report, param, {
        "name", "in", "description", "required", "deprecated",
        "allowEmptyValue", "schema", "content", "style", "explode", "allowReserved"
    }, ctx)
    if not isinstance(param.get("name"), str):
        add_error(report, f"{ctx}.name: must be string")
    param_in = param.get("in")
    if param_in not in {"query", "header", "path", "cookie"}:
        add_error(report, f"{ctx}.in: must be one of query, header, path, cookie")
    if "required" in param and not isinstance(param["required"], bool):
        add_error(report, f"{ctx}.required: must be boolean")
    if param_in == "path" and param.get("required") is not True:
        add_error(report, f"{ctx}.required: must be true for path params")
    if "schema" not in param and "content" not in param:
        add_error(report, f"{ctx}: must include schema or content")

def validate_media_type(report: dict, media: dict, ctx: str, idx: dict[str, set[str]]) -> None:
    if not isinstance(media, dict):
        add_error(report, f"{ctx}: must be object")
        return
    if "$ref" in media:
        if not isinstance(media["$ref"], str) or not ref_exists(media["$ref"], idx):
            add_error(report, f"{ctx}.$ref: unresolved reference")
        return
    if "schema" not in media:
        add_error(report, f"{ctx}: missing schema")
    elif isinstance(media["schema"], dict) and "$ref" in media["schema"]:
        if not isinstance(media["schema"]["$ref"], str) or not ref_exists(media["schema"]["$ref"], idx):
            add_error(report, f"{ctx}.schema.$ref: unresolved reference")

def validate_request_body(report: dict, body: dict, ctx: str, idx: dict[str, set[str]]) -> None:
    if not isinstance(body, dict):
        add_error(report, f"{ctx}: must be object")
        return
    if "$ref" in body:
        if not isinstance(body["$ref"], str) or not ref_exists(body["$ref"], idx):
            add_error(report, f"{ctx}.$ref: unresolved reference")
        return
    content = body.get("content")
    if not isinstance(content, dict) or not content:
        add_error(report, f"{ctx}.content: must be non-empty object")
        return
    for mtype, media in content.items():
        validate_media_type(report, media, f"{ctx}.content[{mtype}]", idx)

def validate_response_obj(report: dict, resp: dict, ctx: str, idx: dict[str, set[str]]) -> None:
    if not isinstance(resp, dict):
        add_error(report, f"{ctx}: must be object")
        return
    if "$ref" in resp:
        if not isinstance(resp["$ref"], str) or not ref_exists(resp["$ref"], idx):
            add_error(report, f"{ctx}.$ref: unresolved reference")
        return
    if not isinstance(resp.get("description"), str):
        add_error(report, f"{ctx}.description: must be string")
    if "content" in resp:
        content = resp["content"]
        if not isinstance(content, dict) or not content:
            add_error(report, f"{ctx}.content: must be non-empty object")
        else:
            for mtype, media in content.items():
                validate_media_type(report, media, f"{ctx}.content[{mtype}]", idx)

def validate_operation(report: dict, op: dict, ctx: str, idx: dict[str, set[str]]) -> None:
    if not isinstance(op, dict):
        add_error(report, f"{ctx}: must be object")
        return
    check_allowed_keys(report, op, {
        "tags", "summary", "description", "operationId", "parameters",
        "requestBody", "responses", "security", "deprecated", "servers",
        "callbacks", "externalDocs"
    }, ctx)
    if not isinstance(op.get("operationId"), str):
        add_error(report, f"{ctx}.operationId: must be string")
    params = op.get("parameters", [])
    if params is not None:
        if not isinstance(params, list):
            add_error(report, f"{ctx}.parameters: must be array")
        else:
            for idx_param, param in enumerate(params):
                pctx = f"{ctx}.parameters[{idx_param}]"
                if isinstance(param, dict) and "$ref" in param:
                    if not isinstance(param["$ref"], str) or not ref_exists(param["$ref"], idx):
                        add_error(report, f"{pctx}.$ref: unresolved reference")
                else:
                    validate_parameter_obj(report, param, pctx)
    if "requestBody" in op:
        validate_request_body(report, op["requestBody"], f"{ctx}.requestBody", idx)
    responses = op.get("responses")
    if not isinstance(responses, dict) or not responses:
        add_error(report, f"{ctx}.responses: must be non-empty object")
    else:
        for code, resp in responses.items():
            validate_response_obj(report, resp, f"{ctx}.responses[{code}]", idx)

def validate_paths(report: dict, paths: dict, idx: dict[str, set[str]]) -> None:
    if not isinstance(paths, dict):
        add_error(report, "paths: must be object")
        return
    allowed_path_keys = {
        "$ref", "summary", "description", "servers", "parameters",
        "get", "put", "post", "delete", "patch", "head", "options", "trace"
    }
    for path, item in paths.items():
        if not isinstance(path, str) or not path.startswith("/"):
            add_error(report, f"paths.{path}: path key must start with '/'")
        if not isinstance(item, dict):
            add_error(report, f"paths.{path}: must be object")
            continue
        check_allowed_keys(report, item, allowed_path_keys, f"paths.{path}")
        if "parameters" in item:
            params = item["parameters"]
            if not isinstance(params, list):
                add_error(report, f"paths.{path}.parameters: must be array")
            else:
                for idx_param, param in enumerate(params):
                    pctx = f"paths.{path}.parameters[{idx_param}]"
                    if isinstance(param, dict) and "$ref" in param:
                        if not isinstance(param["$ref"], str) or not ref_exists(param["$ref"], idx):
                            add_error(report, f"{pctx}.$ref: unresolved reference")
                    else:
                        validate_parameter_obj(report, param, pctx)
        for method in ("get", "put", "post", "delete", "patch", "head", "options", "trace"):
            if method in item:
                validate_operation(report, item[method], f"paths.{path}.{method}", idx)

def validate_components(report: dict, components: dict, idx: dict[str, set[str]]) -> None:
    if not isinstance(components, dict):
        add_error(report, "components: must be object")
        return
    check_allowed_keys(report, components, {"schemas", "responses", "parameters", "securitySchemes"}, "components")
    if "schemas" in components:
        validate_schema_objects(report, components["schemas"])
    if "parameters" in components:
        params = components["parameters"]
        if not isinstance(params, dict):
            add_error(report, "components.parameters: must be object")
        else:
            for name, param in params.items():
                validate_parameter_obj(report, param, f"components.parameters.{name}")
    if "responses" in components:
        responses = components["responses"]
        if not isinstance(responses, dict):
            add_error(report, "components.responses: must be object")
        else:
            for name, resp in responses.items():
                validate_response_obj(report, resp, f"components.responses.{name}", idx)
    if "securitySchemes" in components:
        schemes = components["securitySchemes"]
        if not isinstance(schemes, dict):
            add_error(report, "components.securitySchemes: must be object")
        else:
            for name, scheme in schemes.items():
                if not isinstance(scheme, dict):
                    add_error(report, f"components.securitySchemes.{name}: must be object")
                    continue
                if not isinstance(scheme.get("type"), str):
                    add_error(report, f"components.securitySchemes.{name}.type: must be string")
                if scheme.get("type") == "http" and not isinstance(scheme.get("scheme"), str):
                    add_error(report, f"components.securitySchemes.{name}.scheme: must be string for http")

def validate_refs(report: dict, spec: dict, idx: dict[str, set[str]]) -> None:
    refs: list[str] = []
    def walk(node):
        if isinstance(node, dict):
            for k, v in node.items():
                if k == "$ref" and isinstance(v, str):
                    refs.append(v)
                else:
                    walk(v)
        elif isinstance(node, list):
            for v in node:
                walk(v)
    walk(spec)
    for ref in refs:
        if not ref_exists(ref, idx):
            add_error(report, f"unresolved $ref: {ref}")

def validate_openapi_fallback(report: dict, spec: dict) -> None:
    if not isinstance(spec, dict):
        add_error(report, "spec: must be object")
        return
    allowed_top = {"openapi", "info", "tags", "security", "paths", "components"}
    check_allowed_keys(report, spec, allowed_top, "spec")
    if spec.get("openapi") != "3.1.0":
        add_error(report, "openapi: must be '3.1.0'")
    validate_info(report, spec.get("info"))
    if "tags" in spec:
        validate_tags(report, spec.get("tags"))
    if "security" in spec:
        validate_security(report, spec.get("security"))
    components = spec.get("components", {})
    idx = build_components_index(components) if isinstance(components, dict) else {"schemas": set(), "responses": set(), "parameters": set(), "securitySchemes": set()}
    validate_components(report, components, idx)
    validate_paths(report, spec.get("paths"), idx)
    validate_refs(report, spec, idx)

def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--phase3-zip", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument(
        "--require-secondary",
        action="store_true",
        help="If set, require openapi_spec_validator to be installed and to validate the spec successfully. "
             "By default, the secondary validator is best-effort and any failure is reported as a warning.",
    )
    args = ap.parse_args()

    # NOTE:
    # - Primary validator is always the local, strict, structural + $ref + JSON Schema checks.
    # - Secondary validator (openapi_spec_validator) is best-effort by default, because availability/support
    #   may vary across environments. Failures are recorded under 'warnings' unless --require-secondary is set.
    report = {
        "ok": False,
        "member": None,
        "errors": [],
        "warnings": [],
        # Back-compat: 'validator' remains the primary validator.
        "validator": "fallback_strict",
        "validator_primary": "fallback_strict",
        "validator_secondary": None,
        "validator_secondary_ok": None,
    }

    with zipfile.ZipFile(args.phase3_zip, "r") as z:
        member = find_openapi_member(z)
        report["member"] = member
        spec_bytes = z.read(member)

    # Load YAML (dependency should be present via requirements; fail-closed if missing).
    try:
        import yaml  # type: ignore
    except ModuleNotFoundError as e:
        add_error(report, f"missing dependency: pyyaml ({e})")
        with open(args.out, "w", encoding="utf-8") as f:
            json.dump(report, f, indent=2, sort_keys=True)
        return 2

    try:
        spec = yaml.safe_load(spec_bytes.decode("utf-8"))
    except Exception as e:
        add_error(report, f"YAML parse error: {e}")
        with open(args.out, "w", encoding="utf-8") as f:
            json.dump(report, f, indent=2, sort_keys=True)
        return 2

    # Primary: strict fallback validator (always executed).
    validate_openapi_fallback(report, spec)

    # Secondary: openapi_spec_validator (best-effort unless --require-secondary).
    try:
        from openapi_spec_validator import validate_spec  # type: ignore
        report["validator_secondary"] = "openapi_spec_validator"
        try:
            validate_spec(spec)
            report["validator_secondary_ok"] = True
        except Exception as e:
            report["validator_secondary_ok"] = False
            msg = f"secondary validator (openapi_spec_validator) failed: {e}"
            if args.require_secondary:
                add_error(report, msg)
            else:
                report["warnings"].append(msg)
    except ModuleNotFoundError as e:
        report["validator_secondary"] = "not_installed"
        report["validator_secondary_ok"] = None
        if args.require_secondary:
            add_error(report, f"missing dependency: openapi_spec_validator ({e})")

    report["ok"] = (len(report["errors"]) == 0)

    with open(args.out, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=2, sort_keys=True)
    return 0 if report["ok"] else 2

if __name__ == "__main__":
    raise SystemExit(main())
