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
    // I tried getting this to work by installing a local yapf version,
    // but there seems to be some bugs in pip where it can't use the --target <dir path>
    // on linux without a --system flag. The --system flag then doesn't exist on
    // windows or mac, so it's just a pain and I'm not going to bother with it for now
    let exe_dir_path = get_exe_dir_path();
    let result = Command::new("pip")
            .current_dir(&exe_dir_path)
            .args(&["install", "yapf"])
            // .stderr(Stdio::inherit())
            .output();

    if let Err(err) = result {
        eprintln!("[dprint-plugin-yapf]: Failed to install yapf. You may need to run `pip install yapf` manually. {}", err.to_string());
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
