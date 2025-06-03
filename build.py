from subprocess import Popen
import zipfile
from pathlib import Path

if Path("./release.zip").exists():
    Path("./release.zip").unlink()
    
Popen("cargo build --release", shell=True).wait()

try:
    with zipfile.ZipFile(Path.cwd() / "release.zip", "w") as z:
        for path in (Path.cwd() / "target" / "release").glob("./*.exe"):
            zip_name = f"{path.stem}_RELEASE.exe"
            print(f"Adding {zip_name} to release.zip")
            z.write(path, zip_name)
    print("Release zip created at release.zip.")

except Exception as e:
    print(f"An error occurred: {e}")
    if Path("./release.zip").exists():
        Path("./release.zip").unlink()
    raise e
