#!/usr/bin/env python3
"""Ensure every distribution is built from a consistent release version."""
import os
import sys
import tomllib
from pathlib import Path

root = Path(__file__).resolve().parent.parent
with (root / "Cargo.toml").open("rb") as f:
    cargo_version = tomllib.load(f)["workspace"]["package"]["version"]
with (root / "python/pyproject.toml").open("rb") as f:
    python_version = tomllib.load(f)["project"]["version"]
with (root / "python/uv.lock").open("rb") as f:
    packages = tomllib.load(f)["package"]
lock_versions = [p["version"] for p in packages if p["name"] == "kitsuyui-rust-playground" and p.get("source") == {"editable": "."}]
if python_version != cargo_version or lock_versions != [cargo_version]:
    sys.exit(f"Version mismatch: Cargo={cargo_version}, Python={python_version}, Python lock={lock_versions}")
if os.environ.get("GITHUB_EVENT_NAME") == "release":
    tag = os.environ.get("GITHUB_REF_NAME", "")
    if tag != f"v{cargo_version}":
        sys.exit(f"Release tag {tag!r} does not match v{cargo_version}")
print(f"Distribution versions agree: {cargo_version}")
