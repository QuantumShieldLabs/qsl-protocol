from pathlib import Path
import xml.etree.ElementTree as ET

def write_junit(path: Path, suite_name: str, cases: list[dict]) -> None:
    ts = ET.Element("testsuite", name=suite_name)
    failures = 0
    tests = 0
    for c in cases:
        if c.get("status") not in ("passed", "failed"):
            continue
        tests += 1
        tc = ET.SubElement(ts, "testcase", name=c["id"])
        if c["status"] == "failed":
            failures += 1
            f = ET.SubElement(tc, "failure", message="failed")
            f.text = str({"expected": c.get("expected"), "actual": c.get("actual")})
    ts.set("tests", str(tests))
    ts.set("failures", str(failures))
    tree = ET.ElementTree(ts)
    path.parent.mkdir(parents=True, exist_ok=True)
    tree.write(path, encoding="utf-8", xml_declaration=True)
