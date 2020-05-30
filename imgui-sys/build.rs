#![allow(dead_code)]

use std::fs;
use std::io;

const CPP_FILES: [&str; 5] = [
    "third-party/cimgui/cimgui.cpp",
    "third-party/cimgui/imgui/imgui.cpp",
    "third-party/cimgui/imgui/imgui_demo.cpp",
    "third-party/cimgui/imgui/imgui_draw.cpp",
    "third-party/cimgui/imgui/imgui_widgets.cpp",
];

fn assert_file_exists(path: &str) -> io::Result<()> {
    match fs::metadata(path) {
        Ok(_) => Ok(()),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            panic!(
                "Can't access {}. Did you forget to fetch git submodules?",
                path
            );
        }
        Err(e) => Err(e),
    }
}

fn main() -> io::Result<()> {
    // #[cfg(not(feature = "wasm"))]
    {
        let mut build = cc::Build::new();
        build.cpp(true);

        
        let target = std::env::var("TARGET").unwrap();
        if target.starts_with("wasm32") {
            // In webassembly there's no stdlib, so we use
            // our own stripped down headers to provide the few
            // functions needed via LLVM intrinsics.
            build.flag("-isystem").flag("wasm-sysroot");
            // The Wasm backend needs a compatible ar
            // which will most likely be available under
            // this name on Windows, via manual LLVM install
            let host = std::env::var("HOST").unwrap();
            if host.contains("windows") {
                build.archiver("llvm-ar");
            }

            // too many warning, it's annoying
            build.flag("-Wno-return-type-c-linkage");
        }

        // Disabled due to linking issues
        build
            .define("CIMGUI_NO_EXPORT", None)
            .define("IMGUI_DISABLE_WIN32_FUNCTIONS", None)
            .define("IMGUI_DISABLE_OSX_FUNCTIONS", None);

        build.flag_if_supported("-Wno-return-type-c-linkage");
        for path in &CPP_FILES {
            assert_file_exists(path)?;
            build.file(path);
        }
        build.compile("libcimgui.a");
    }
    Ok(())
}
