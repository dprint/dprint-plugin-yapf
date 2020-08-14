use std::process::{Command, Stdio};
use std::path::PathBuf;

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
    Command::new("python")
        .current_dir(&exe_dir_path)
        .args(&["-u", "main.py"]) // -u for unbuffered stdin/stdout
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("failed to run python on path");
}

fn init() {
    let exe_dir_path = get_exe_dir_path();
    let _ = Command::new("mkdir")
            .current_dir(&exe_dir_path)
            .args(&["packages"]);
    let result = Command::new("pip")
            .current_dir(&exe_dir_path)
            // needs --system because otherwise ubuntu errors
            // https://github.com/pypa/pip/issues/3826#issuecomment-427622702
            .args(&["install", "-Iv", "yapf==0.30.0", "--target", "packages", "--system"])
            .stderr(Stdio::inherit())
            .output();

    if let Err(err) = result {
        eprintln!("[dprint-plugin-yapf]: {}", err.to_string());
        panic!("[dprint-plugin-yapf]: Failed to install yapf.");
    }
}

fn get_exe_dir_path() -> PathBuf {
    let exe_file_path = std::env::current_exe().expect("expected to get the executable file path");
    let exe_dir_path = exe_file_path.parent().expect("expected to get executable directory path");
    exe_dir_path.to_path_buf()
}

fn is_init(args: &Vec<String>) -> bool {
    for arg in args {
        if arg == "--init" {
            return true;
        }
    }

    false
}

fn get_parent_process_id_from_args(args: &Vec<String>) -> u32 {
    for i in 0..args.len() {
        if args[i] == "--parent-pid" {
            if let Some(parent_pid) = args.get(i + 1) {
                return parent_pid.parse::<u32>().expect("could not parse the parent process id");
            }
        }
    }

    panic!("please provide a --parent-pid <id> flag")
}
