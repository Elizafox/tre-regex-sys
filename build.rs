// SPDX-License-Identifier: BSD-2-Clause
// See LICENSE file in the project root for full license text.

extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let include_path;
    #[allow(unused_mut)]
    let mut clang_args: Vec<String> = Vec::new();

    #[cfg(feature = "vendored")]
    {
        use autotools::Config;
        use fs_extra::dir::{CopyOptions, copy, remove};
        use std::process::Command;

        let tre_path = out_path.join("tre");

        // Clean up if we have to
        remove(&tre_path).ok();

        // Sigh, so tre does weird build stuff and we can't "just" run configure.
        // We have to run the generation script, but we aren't supposed to modify outside OUT_DIR, so,
        // we just copy it to out dir. Disgusting, I know. If you have a better option, please let me
        // know.
        let options = CopyOptions::new();
        copy("tre", &out_path, &options).expect("Failed to copy tre!");

        // Generate the config script.
        // I hate autotools so much.
        let status = Command::new("sh")
            .current_dir(&tre_path)
            .args(["-c", "./utils/autogen.sh"])
            .status()
            .expect("Could not run autogen.sh! Is autotools installed?");
        assert!(status.success(), "autogen.sh failed with {status}");

        // This is messy, I know, but rustc complains otherwise.
        let mut cfg = Config::new(&tre_path);
        let dst = cfg.enable_static().disable_shared().disable("agrep", None);

        let dst = if cfg!(feature = "wchar") {
            dst.enable("wchar", None)
        } else {
            dst.disable("wchar", None)
        };

        let dst = if cfg!(feature = "approx") {
            dst.enable("approx", None)
        } else {
            dst.disable("approx", None)
        };

        let dst = dst.build();

        // Clean up our mess
        remove(tre_path).expect("Could not clean up tre dir!");

        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib").display()
        );
        println!("cargo:rustc-link-lib=static=tre");
        println!("cargo:rustc-link-lib=c");

        let pathbuf = out_path.join("include").join("tre").join("tre.h");
        include_path = pathbuf.to_str().unwrap().to_string();
    }
    #[cfg(not(feature = "vendored"))]
    {
        let library = pkg_config::Config::new().statik(true).find("tre").unwrap();
        clang_args.extend(
            library
                .include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy()))
                .collect::<Vec<_>>(),
        );
        include_path = "sys-wrapper.h".to_string();
    }

    let mut bindings = bindgen::Builder::default()
        .clang_args(clang_args)
        .header(include_path)
        .derive_default(true)
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed) // most API's use this
        .newtype_enum("reg_errcode_t")
        .opaque_type("regex_t") // Should be opaque
        .allowlist_function("tre_.*")
        .allowlist_type("(reg.*_t|tre_.*)")
        .allowlist_var("REG_.*")
        .blocklist_type("register_t");

    if !cfg!(feature = "wchar") {
        bindings = bindings.blocklist_function("tre_reg(a)?w(n)?(comp|exec)");
    }

    if !cfg!(feature = "approx") {
        bindings = bindings
            .blocklist_function("tre_rega(w)?(n)?exec")
            .blocklist_type("rega(match|params)_t")
            .blocklist_item("REG_APPROX_MATCHER");
    }

    let bindings = bindings.generate().expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
