#!/usr/bin/env python3
"""Create a deterministic .tar.gz or .zip of a directory (stdlib only).

Entries are sorted, owned by 0:0, timestamped SOURCE_DATE_EPOCH (default 0;
zip clamps to 1980), and keep only the executable bit of the source mode.
The archive's single top-level directory is the source directory's name.

Usage: archive.py <src-dir> <out.tar.gz|out.zip>
"""

import gzip
import io
import os
import sys
import tarfile
import time
import zipfile
from pathlib import Path


def entries(src: Path):
    yield src
    for path in sorted(src.rglob("*"), key=lambda p: p.relative_to(src).as_posix()):
        yield path


def mode_of(path: Path) -> int:
    if path.is_dir() or os.access(path, os.X_OK):
        return 0o755
    return 0o644


def write_tar_gz(src: Path, out: Path, epoch: int) -> None:
    raw = io.BytesIO()
    with tarfile.open(fileobj=raw, mode="w", format=tarfile.PAX_FORMAT) as tar:
        for path in entries(src):
            arcname = (Path(src.name) / path.relative_to(src)).as_posix()
            info = tar.gettarinfo(str(path), arcname)
            info.uid = info.gid = 0
            info.uname = info.gname = ""
            info.mtime = epoch
            info.mode = mode_of(path)
            if info.isreg():
                with open(path, "rb") as fh:
                    tar.addfile(info, fh)
            else:
                tar.addfile(info)
    with open(out, "wb") as fh, gzip.GzipFile(fileobj=fh, mode="wb", mtime=epoch, filename="") as gz:
        gz.write(raw.getvalue())


def write_zip(src: Path, out: Path, epoch: int) -> None:
    stamp = time.gmtime(max(epoch, 315532800))[:6]  # zip epoch is 1980-01-01
    with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as zf:
        for path in entries(src):
            arcname = (Path(src.name) / path.relative_to(src)).as_posix()
            if path.is_dir():
                arcname += "/"
            info = zipfile.ZipInfo(arcname, stamp)
            info.external_attr = (mode_of(path) | (0o040000 if path.is_dir() else 0o100000)) << 16
            info.compress_type = zipfile.ZIP_DEFLATED
            zf.writestr(info, b"" if path.is_dir() else path.read_bytes())


def main(argv: list[str]) -> int:
    if len(argv) != 2:
        print(__doc__, file=sys.stderr)
        return 64
    src, out = Path(argv[0]).resolve(), Path(argv[1])
    epoch = int(os.environ.get("SOURCE_DATE_EPOCH", "0"))
    match out.name:
        case name if name.endswith(".tar.gz"):
            write_tar_gz(src, out, epoch)
        case name if name.endswith(".zip"):
            write_zip(src, out, epoch)
        case _:
            print(f"unsupported archive type: {out}", file=sys.stderr)
            return 64
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
