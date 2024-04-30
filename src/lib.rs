use clap::Parser;
use std::net::TcpStream;
use std::process::Command;
use std::thread;
use std::time::Duration;
use tokio::time::timeout;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 30)]
    timeout: u64,

    #[arg(short, long = "addr", required = true, value_name = "ADDRESS")]
    addresses: Vec<String>,

    #[arg(
        required = true,
        last = true,
        allow_hyphen_values = true,
        value_name = "COMMAND"
    )]
    cmd: Vec<String>,

    #[arg(short = 'q', long)]
    quiet: bool,
}

async fn wait_for(addresses: Vec<String>, quiet: bool) {
    let mut threads = Vec::new();
    for address in addresses {
        let thread = tokio::spawn(async move {
            loop {
                match TcpStream::connect(&address) {
                    Ok(_) => {
                        if !quiet {
                            println!("Connected to {} successfully", address);
                        }
                        break;
                    }
                    Err(_) => {
                        if !quiet {
                            println!("Waiting for {}", address)
                        }
                    }
                }
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.await.unwrap_or_default();
    }
}

pub async fn run() -> Result<(), &'static str> {
    let args = Args::parse();

    let thread = thread::spawn(move || async move {
        let my_duration = tokio::time::Duration::from_secs(args.timeout);
        timeout(my_duration, wait_for(args.addresses, args.quiet)).await
    });

    if thread.join().unwrap().await.is_err() {
        return Err("Connection timeout, could not connect to the addresses.");
    }

    Command::new(&args.cmd[0])
        .args(&args.cmd[1..])
        .spawn()
        .expect("Failed to run the command.");

    Ok(())
}
