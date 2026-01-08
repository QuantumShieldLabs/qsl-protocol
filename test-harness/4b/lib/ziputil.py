from pathlib import Path
import zipfile

def read_member_bytes(zip_path: Path, member: str) -> bytes:
    with zipfile.ZipFile(zip_path, "r") as z:
        return z.read(member)

def extract_member_to(zip_path: Path, member: str, out_path: Path) -> None:
    out_path.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(zip_path, "r") as z:
        with z.open(member, "r") as src, out_path.open("wb") as dst:
            dst.write(src.read())
