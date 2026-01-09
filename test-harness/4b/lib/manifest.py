from pathlib import Path
from .hashutil import sha256_file
import json
import mimetypes
import time

def write_manifest_4b(artifacts_dir: Path, out_dir: Path, run_id: str, git_commit: str) -> None:
    files = []
    for p in sorted(artifacts_dir.rglob("*")):
        if p.is_dir():
            continue
        rel = p.relative_to(artifacts_dir).as_posix()
        mt = mimetypes.guess_type(p.name)[0] or "application/octet-stream"
        files.append({
            "path": rel,
            "sha256": sha256_file(p),
            "bytes": p.stat().st_size,
            "media_type": mt,
            "role": "evidence",
            "required": True
        })
    obj = {
        "format": "QSHIELD-4B-MANIFEST-1",
        "created_at": int(time.time()),
        "run_id": run_id,
        "git_commit": git_commit,
        "files": files
    }
    out_path = out_dir / "B4_manifest.json"
    out_path.write_text(json.dumps(obj, sort_keys=True, indent=2) + "\n", encoding="utf-8")
