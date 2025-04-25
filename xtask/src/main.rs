use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No xtask command given. Available: install-hooks");
        exit(1);
    }

    match args[1].as_str() {
        "install-hooks" => install_hooks(),
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

        // Symlink if possible, fallback to copy
        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            let _ = fs::remove_file(&dest);
            symlink(&src, &dest).expect("failed to symlink hook");
        }

        #[cfg(windows)]
        {
            // Windows needs explicit copy if no symlink privileges
            fs::copy(&src, &dest).expect("failed to copy hook");
        }

        println!("Installed {:?}", hook_name);
    }
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
