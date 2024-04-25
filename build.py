from pathlib import Path
from subprocess import Popen, PIPE, TimeoutExpired
import sys
import zipfile
import argparse
BASE_CARGO_TOML = """[package]
name = "tmp_build"
version = "0.1.0"
edition = "2018"

[dependencies]
nannou = "0.19.0"
"""
BASE_RUST_MAIN = """use nannou::prelude::*;
fn main() {
    nannou::app(model).run();
}
struct Model {}
fn model(_app: &App) -> Model {
    Model {}
    }
"""
def get_default_cargo_target_triple() -> str:
    proc = Popen(
        ["rustc", "--version", "--verbose"],
        stdout=PIPE,
        stderr=PIPE,
        text=True,
    )
    out, err = proc.communicate(timeout=5)
    if proc.returncode != 0:
        raise RuntimeError(f"Failed to get rustc version: {err}")
    for line in out.splitlines():
        if line.startswith("host: "):
            return line.split()[1]
    raise RuntimeError("Failed to get host target triple from rustc output.")

parser = argparse.ArgumentParser()
parser.add_argument(
    "--release",
    action="store_true",
    help="Build in release mode. Default is debug mode.",
    default=True,
)
parser.add_argument(
    "--target",
    type=str,
    help="Build for the specified target.",
    default=get_default_cargo_target_triple(),
)
dirs = parser.add_mutually_exclusive_group()
dirs.add_argument(
    "--all",
    action="store_true",
    help="Build all directories in the current directory.",
)
dirs.add_argument(
    "--dir",
    type=str,
    help="Build the specified directory.",
)
args = parser.parse_args()


BUILD_SUFFIX = Path("target/release")
ROOT = Path.cwd()
RELEASE = ROOT / "release"
TMP_BUILD = ROOT / "tmp_build"

success_paths: list[Path] = []
dirs = Path.cwd().iterdir() if args.all else [Path(args.dir)] if args.dir else []

# Construct the build command
cmd = ["cargo", "build", "--color", "never", "--message-format", "short", "--target", args.target]
if args.release:
    cmd.append("--release")
    
# populate the temporary build directory
TMP_BUILD.mkdir(exist_ok=True)
(TMP_BUILD / "src").mkdir(exist_ok=True)
(TMP_BUILD / "src" / "main.rs").write_text(BASE_RUST_MAIN)
(TMP_BUILD / "target").mkdir(exist_ok=True)
(TMP_BUILD/"Cargo.toml").write_text(BASE_CARGO_TOML)
print("Building nannou and dependencies")
Popen(
    cmd,
    stdout=sys.stdout,
    stderr=sys.stderr,
    cwd=TMP_BUILD,
).wait()
print("Build complete.")

# Build the directories
print(f"Building with command: {' '.join(cmd)}")
for path in dirs:
    if path.is_dir():
        # check if the directory contains a Cargo.toml file
        if not (path / "Cargo.toml").exists():
            print(f"Skipping {path} as it does not contain a Cargo.toml file.")
            continue
        print(f"Building directory: {path}")
        
        # clean the temporary build directory
        for file in TMP_BUILD.glob("src/*"):
            file.unlink(missing_ok=True)
        (TMP_BUILD / "Cargo.toml").unlink(missing_ok=True)
        
        # copy the source files to the temporary build directory
        for src_file in path.glob("src/*"):
            (TMP_BUILD / src_file.relative_to(path)).resolve().write_text(src_file.read_text())
        (TMP_BUILD/"Cargo.toml").write_text((path/ "Cargo.toml").read_text())
        
        p = Popen(
                cmd,
                stdout=sys.stdout,
                stderr=sys.stderr,
                cwd=TMP_BUILD,
            )
        
        if p.wait() != 0:
            print(f"Failed to build {path}. Exiting.")
            sys.exit(1)
            
        success_paths.append(path)
            


for path in success_paths:
    exe_path = (path / BUILD_SUFFIX / path.stem).with_suffix(".exe")
    out_path = (RELEASE / path.stem).with_suffix(".exe")
    if not out_path.parent.exists():
        out_path.parent.mkdir(parents=True)
    elif out_path.exists():
        out_path.unlink()
    elif not exe_path.exists():
        print(f"Skipping {exe_path.relative_to(Path.cwd())} as it does not exist.")
        continue
    else:
        print(
            f"Copying {exe_path.relative_to(Path.cwd())} to {out_path.relative_to(Path.cwd())}"
        )
        out_path.write_bytes(exe_path.read_bytes())

with zipfile.ZipFile(ROOT / "release.zip", "w") as z:
    for path in RELEASE.iterdir():
        if path.is_file() and path.suffix == ".exe":
            zip_name = f"{path.stem}{("_"+args.target.replace("-","_")) if args.target else ""}{("_RELEASE" if args.release else "")}.exe"
            print(f"Adding {zip_name} to release.zip")
            z.write(path, zip_name)

print("Release zip created at release.zip.")