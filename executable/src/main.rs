use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

mod parent_process_checker;
use parent_process_checker::start_parent_process_checker_thread;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parent_pid = get_parent_process_id_from_args(&args);
    let _ = start_parent_process_checker_thread("dprint-plugin-yapf".to_string(), parent_pid);

    if is_init(&args) {
        init();
    }

    let exe_dir_path = get_exe_dir_path();
    Command::new(get_python_program())
        .current_dir(&exe_dir_path)
        .args(&["-u", "main.py"]) // -u for unbuffered stdin/stdout
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("failed to run python on path");
}

fn init() {
    eprintln!("Installing yapf...");

    // Install the latest version of pip to a temporary directory.
    // This is necessary because the version bundled with python is too old to
    // do `--target <dir path>` (buggy and doesn't work reliably across systems).
    // See https://github.com/denoland/deno/blob/5f1df038fb1462607af3555fa7431c05ca484dce/tools/third_party.py#L61
    let exe_dir_path = get_exe_dir_path();
    let temp_python_user_base_dir = exe_dir_path.join("temp");
    fs::create_dir_all(&temp_python_user_base_dir).unwrap_or_else(|_| {
        panic!(
            "Expected to be able to create a temporary directory at {}",
            temp_python_user_base_dir.display()
        )
    });

    Command::new(get_python_program())
        .current_dir(&exe_dir_path)
        .env("PYTHONUSERBASE", &temp_python_user_base_dir)
        .args(&["-m", "pip", "install", "--user", "pip==22.0.2"])
        .stderr(Stdio::inherit())
        .output()
        .expect("Error installing pip locally.");

    // Install yapf to a local `packages` directory
    let packages_dir = exe_dir_path.join("packages");
    fs::create_dir_all(&packages_dir).unwrap_or_else(|_| {
        panic!(
            "Expected to be able to create a directory at {}",
            packages_dir.display()
        )
    });
    Command::new(get_python_program())
        .current_dir(&exe_dir_path)
        .env("PYTHONUSERBASE", &temp_python_user_base_dir)
        .args(&[
            "-m",
            "pip",
            "install",
            "--target",
            &packages_dir.to_string_lossy(),
            "yapf==0.32.0",
        ])
        .stderr(Stdio::inherit())
        .output()
        .expect("Error installing yapf locally.");

    fs::remove_dir_all(&temp_python_user_base_dir).expect("Error removing temp directory.");
}

fn get_exe_dir_path() -> PathBuf {
    let exe_file_path = std::env::current_exe().expect("expected to get the executable file path");
    let exe_dir_path = exe_file_path
        .parent()
        .expect("expected to get executable directory path");
    exe_dir_path.to_path_buf()
}

fn is_init(args: &[String]) -> bool {
    for arg in args {
        if arg == "--init" {
            return true;
        }
    }

    false
}

fn get_parent_process_id_from_args(args: &[String]) -> u32 {
    for i in 0..args.len() {
        if args[i] == "--parent-pid" {
            if let Some(parent_pid) = args.get(i + 1) {
                return parent_pid
                    .parse::<u32>()
                    .expect("could not parse the parent process id");
            }
        }
    }

    panic!("please provide a --parent-pid <id> flag")
}
fn get_python_program() -> String {
    std::env::var("DPRINT_YAPF_PYTHON_CMD").unwrap_or_else(|_| "python".to_string())
}
