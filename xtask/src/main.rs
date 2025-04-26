use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "No xtask command given. Available: install-hooks, install-tools"
        );
        exit(1);
    }

    match args[1].as_str() {
        "install-hooks" => install_hooks(),
        "install-tools" => install_tools(),
        _ => {
            eprintln!("Unknown xtask command: {}", args[1]);
            exit(1);
        },
    }
}

fn install_hooks() {
    let repo_root = project_root();
    let hooks_dir = repo_root.join(".workspace/hooks");
    let git_hooks_dir = repo_root.join(".git/hooks");

    println!("Installing git hooks...");
    for entry in fs::read_dir(&hooks_dir).expect("failed to read hooks dir") {
        let entry = entry.expect("invalid dir entry");
        let src = entry.path();
        let hook_name = src.file_name().expect("no filename").to_owned();
        let dest = git_hooks_dir.join(&hook_name);

        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            let _ = fs::remove_file(&dest);
            symlink(&src, &dest).expect("failed to symlink hook");
        }

        #[cfg(windows)]
        {
            fs::copy(&src, &dest).expect("failed to copy hook");
        }

        println!("Installed {:?}", hook_name);
    }
}

fn install_tools() {
    if !is_tool_installed("cargo-llvm-cov") {
        println!("Installing cargo-llvm-cov...");
        let status = Command::new("cargo")
            .args(["install", "cargo-llvm-cov"])
            .status()
            .expect("failed to execute cargo install cargo-llvm-cov");
        if !status.success() {
            eprintln!("Failed to install cargo-llvm-cov");
            exit(1);
        }
    } else {
        println!("cargo-llvm-cov already installed");
    }
}

fn is_tool_installed(tool_name: &str) -> bool {
    let output = Command::new("cargo")
        .args(["install", "--list"])
        .output()
        .expect("failed to list installed cargo tools");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.contains(tool_name)
}

fn project_root() -> PathBuf {
    let mut dir = env::current_dir().expect("cannot get current dir");
    while !dir.join("Cargo.toml").exists() {
        if !dir.pop() {
            panic!("Cannot find Cargo.toml in parent directories");
        }
    }
    dir
}
