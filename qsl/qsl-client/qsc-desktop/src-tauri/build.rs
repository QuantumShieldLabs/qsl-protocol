use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("cargo manifest dir"));
    let resource_dir = manifest_dir.join("resources").join("bin");
    let sidecar_path = resource_dir.join("qsc");
    let stub_path = resource_dir.join("qsc.stub");

    println!("cargo:rerun-if-changed={}", stub_path.display());

    if !sidecar_path.exists() {
        fs::create_dir_all(&resource_dir).expect("resource bin dir");
        fs::copy(&stub_path, &sidecar_path).expect("copy qsc stub");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&sidecar_path, fs::Permissions::from_mode(0o755))
                .expect("chmod qsc stub");
        }
    }

    tauri_build::build()
}
