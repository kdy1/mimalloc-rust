use std::env;

fn main() {
    let mut build = cc::Build::new();

    build.include("./mimalloc/include");
    build.include("./mimalloc/src");
    build.file("./mimalloc/src/static.c");

    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("target_os not defined!");
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").expect("target_family not defined!");

    if env::var_os("CARGO_FEATURE_OVERRIDE").is_some() {
        // Overriding malloc is only available on windows in shared mode, but we
        // only ever build a static lib.
        if target_family != "windows" {
            build.define("MI_MALLOC_OVERRIDE", None);
        }
    }

    if env::var_os("CARGO_FEATURE_SECURE").is_some() {
        build.define("MI_SECURE", "4");
    }

    let dynamic_tls = env::var("CARGO_FEATURE_LOCAL_DYNAMIC_TLS").is_ok();

    if target_family == "unix" && target_os != "haiku" {
        if dynamic_tls {
            build.flag_if_supported("-ftls-model=local-dynamic");
        } else {
            build.flag_if_supported("-ftls-model=initial-exec");
        }
    }

    // Remove heavy debug assertions etc
    let profile = std::env::var("PROFILE").unwrap();
    match profile.as_str() {
        "debug" => build.define("MI_DEBUG", "3"),
        "release" => build.define("MI_DEBUG", "0"),
        _ => build.define("MI_DEBUG", "3"),
    };

    if build.get_compiler().is_like_msvc() {
        build.cpp(true);
    }

    build.compile("mimalloc");
}
