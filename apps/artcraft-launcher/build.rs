//! Windows only: embed the app icon and version info (VERSIONINFO) into `freeandiy-launcher.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `FREEANDIY_LAUNCHER_REQUIRE_WINRES=1` (set
//! by the release packaging) turns it into an error.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/freeandiy-launcher.ico");
    println!("cargo:rerun-if-env-changed=FREEANDIY_LAUNCHER_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/freeandiy-launcher.ico")
        .set("ProductName", "FreeANDIY Launcher")
        .set("FileDescription", "FreeANDIY Launcher")
        .set("CompanyName", "Free Andy Agency")
        .set("LegalCopyright", "Copyright (c) the FreeANDIY Launcher authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "freeandiy-launcher.exe")
        .set("InternalName", "freeandiy-launcher");
    if let Err(e) = res.compile() {
        if std::env::var_os("FREEANDIY_LAUNCHER_REQUIRE_WINRES").is_some() {
            println!("cargo:warning=embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=freeandiy-launcher.exe built without icon/version resources: {e}");
    }
}
