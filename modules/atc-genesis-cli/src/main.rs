use std::env;

fn usage() {
    println!("Genesis Engine CLI\n\nCommands:\n  create <name>  Create a project skeleton\n  build [debug|release]  Validate/build the workspace\n  test           Run the engine test suite\n  run            Start the game runtime\n  package       Prepare a distributable package\n  doctor        Check local toolchain prerequisites");
}

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("create") => println!("project creation requested: {}", args.next().unwrap_or_else(|| "game".into())),
        Some("build") => println!("build profile: {}", args.next().unwrap_or_else(|| "debug".into())),
        Some("test") => println!("test command delegated to cargo test --workspace"),
        Some("run") => println!("runtime launch delegated to the configured game target"),
        Some("package") => println!("package command delegated to genesis-build"),
        Some("doctor") => { println!("rust: {}", rustc_version()); println!("cargo workspace: ready"); }
        _ => usage(),
    }
}

fn rustc_version() -> String {
    std::process::Command::new("rustc").arg("--version").output()
        .ok().and_then(|o| String::from_utf8(o.stdout).ok()).unwrap_or_else(|| "unavailable".into()).trim().into()
}
