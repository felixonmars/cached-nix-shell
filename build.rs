use std::env::{var, var_os};
use std::process::Command;

fn main() {
    let out_dir = var("OUT_DIR").unwrap();

    let cmd = Command::new("gcc")
        .args(&[
            "-fPIC",
            "-shared",
            "-o",
            &format!("{}/trace-nix.so", out_dir),
            "nix-trace/trace-nix.c",
        ])
        .status()
        .unwrap();
    assert!(cmd.success());

    if var_os("CNS_IN_NIX_BUILD").is_some() {
        let out = var("out").unwrap();
        println!("cargo:rustc-env=CNS_TRACE_NIX_SO={}/lib/trace-nix.so", out);
        println!("cargo:rustc-env=CNS_VAR_EMPTY={}/var/empty", out);
        println!(
            "cargo:rustc-env=CNS_RCFILE={}/share/cached-nix-shell/rcfile.sh",
            out
        );
    } else {
        println!("cargo:rustc-env=CNS_TRACE_NIX_SO={}/trace-nix.so", out_dir);
        println!("cargo:rustc-env=CNS_VAR_EMPTY=/var/empty");
        println!(
            "cargo:rustc-env=CNS_RCFILE={}/rcfile.sh",
            var("CARGO_MANIFEST_DIR").unwrap()
        );
    }
}
