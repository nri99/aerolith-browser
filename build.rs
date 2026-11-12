fn main() {
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    if target_env == "gnu" {
        // Dynamically resolve MinGW library search path if available in cargo home
        if let Ok(cargo_home) = std::env::var("CARGO_HOME") {
            let p = std::path::PathBuf::from(cargo_home).join("mingw64").join("lib");
            if p.exists() {
                println!("cargo:rustc-link-search=native={}", p.display());
            }
        } else if let Ok(user_profile) = std::env::var("USERPROFILE") {
            let p = std::path::PathBuf::from(user_profile)
                .join(".cargo")
                .join("mingw64")
                .join("lib");
            if p.exists() {
                println!("cargo:rustc-link-search=native={}", p.display());
            }
        }

        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=oleaut32");
        println!("cargo:rustc-link-lib=shell32");
        println!("cargo:rustc-link-lib=dwmapi");
        println!("cargo:rustc-link-lib=uxtheme");
        println!("cargo:rustc-link-lib=comctl32");
        println!("cargo:rustc-link-lib=shlwapi");
        println!("cargo:rustc-link-lib=propsys");
        println!("cargo:rustc-link-lib=rpcrt4");
        println!("cargo:rustc-link-lib=imm32");
        println!("cargo:rustc-link-lib=uuid");
        println!("cargo:rustc-link-lib=msimg32");
        println!("cargo:rustc-link-lib=dcomp");
    }
}
