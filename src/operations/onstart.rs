use std::{fs, io, process};

fn shell(shell: &str, command: Vec<&str>) {
    process::Command::new(shell)
        .arg("-c")
        .args(command.as_slice())
        .spawn()
        .expect("test");
}

fn check_shell() -> Option<&'static str> {
    // use POSIX compliant shells only.
    let shfile = fs::read_to_string("/etc/shells").ok()?;
    let shells: Vec<&str> = shfile.split("\n").collect();
    let valid_shells = [
        "bash", "sh", "zsh", "ksh", "dash", "nsh", "osh", "yash", "tcsh",
    ];
    let mut chosen_shell: Option<&str> = None;
    for shell in valid_shells {
        let pattern = format!("/usr/bin/{}", shell);
        if shells.contains(&pattern.as_str()) {
            println!("found shell: {shell}");
            chosen_shell = Some(shell);
            break;
        }
    }
    chosen_shell
}

pub fn init() {
    // on run... do this:
    /*
     * 1. Detect what shells are on the machine
     * 2. print a nice little hello message
     * 3. see what other computers are running on a lan.
     * 4. print a goodbye message.
     * */
    println!("Hello there, i am malware!");
    let shellcommand = match check_shell() {
        Some(x) => x,
        None => "not found...",
    };
    //tokio can be used to spawn a server to create a reverse shell.
    let command = vec!["echo this malware world"];
    shell(shellcommand, command);
}
