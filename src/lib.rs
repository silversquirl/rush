use std::process::Command;
use std::io::{Write, stdin, stdout, stderr};

enum SplitState {
    DQString,
    SQString,
    NoString
}
use SplitState::{DQString, SQString, NoString};

fn split_commandline(commandline: String) -> (String, Vec<String>) {
    let mut state = NoString;
    let mut command = String::new();
    let mut args = Vec::new();
    let mut tmp = String::new();
    let mut first = true;

    for c in commandline.chars() {
        match c {
            ' ' => {
                if let NoString = state {
                    if first {
                        first = false;
                        command = tmp;
                    } else {
                        args.push(tmp);
                    }
                    tmp = String::new();
                } else {
                    tmp.push(c);
                }
            },
            '"' => {
                match state {
                    DQString => state = NoString,
                    SQString => tmp.push(c),
                    NoString => state = DQString
                }
            },
            '\'' => {
                match state {
                    DQString => tmp.push(c),
                    SQString => state = NoString,
                    NoString => state = SQString
                }
            },
            c => tmp.push(c)
        }
    }

    if first {
        command = tmp;
    } else {
        args.push(tmp);
    }

    (command, args)
}

pub fn run_command(commandline: String) {
    let (cmd_name, args) = split_commandline(commandline);
    let status = Command::new(cmd_name.clone()).args(&args).status();
    if let Err(_) = status {
        writeln!(
            stderr(),
            "rush: {}: command not found",
            cmd_name).unwrap();
    }
}

pub fn get_line(prompt: &str) -> Option<String> {
    print!("{}", prompt);
    stdout().flush().unwrap();

    let mut line = String::new();
    if stdin().read_line(&mut line).unwrap() > 0 {
        line.pop();
        Some(line)
    } else {
        println!("");
        None
    }
}

