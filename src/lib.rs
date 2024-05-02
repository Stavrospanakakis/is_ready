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

fn exec_command(command: &str, args: &[String]) -> Result<(), String> {
    if Command::new(command).args(args).spawn().is_err() {
        let err = format!("Command not found: {}", command);
        return Err(err);
    }

    Ok(())
}

pub async fn run() -> Result<(), String> {
    let args = Args::parse();

    let thread = thread::spawn(move || async move {
        let my_duration = tokio::time::Duration::from_secs(args.timeout);
        timeout(my_duration, wait_for(args.addresses, args.quiet)).await
    });

    if thread.join().unwrap().await.is_err() {
        return Err(String::from(
            "Connection timeout, could not connect to the addresses.",
        ));
    }

    exec_command(&args.cmd[0], &args.cmd[1..])?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_not_found() {
        let args = Args {
            timeout: 30,
            addresses: vec![String::from("google.com:80")],
            cmd: vec![String::from("not_a_command")],
            quiet: false,
        };

        assert_eq!(
            exec_command(&args.cmd[0], &args.cmd[1..]),
            Err(String::from("Command not found: not_a_command"))
        );
    }
}
