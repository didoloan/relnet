mod options;
mod stdin_fd;
mod telnet_sock;

use async_lock::OnceCell;
use clap::Parser;
use monoio::io::{AsyncBufReadExt, AsyncWriteRent, AsyncWriteRentExt, Splitable};
use options::Cli;
use stdin_fd::StdInFd;
use telnet_sock::TelnetSock;


static IS_CLOSING:OnceCell<bool> = OnceCell::new();

#[monoio::main(enable_timer = true)]
async fn main() {
    let res = run_telnet().await;

    eprintln!("{}", res.unwrap_err());
    
    ()
}

async fn run_telnet() -> Result<(), Box<dyn std::error::Error>> {

    let cli = Cli::parse();

    println!("{:?}", cli);

    let client_conn = TelnetSock::connect(cli.get_addrs(), cli.secure).await?;

    let (read_half, mut write_half) = client_conn.into_split();

    let handle = monoio::spawn(async {
        let mut reader = monoio::io::BufReader::new(read_half);

        let mut buf = String::new();

        while let Ok(line) = reader.read_line(&mut buf).await {
            if line > 0 {
                print!("{}", buf);
                buf.clear();
            } else {
                println!("{}", "Connection closed by foreign host.");
                let _ = IS_CLOSING.set(true).await;
                break;
            }
        }
    });

    let mut buf_stdin = monoio::io::BufReader::new(StdInFd::new()?);

    loop {
        if IS_CLOSING.is_initialized() {
            let _ = write_half.shutdown().await;
            break;
        }
        let mut buf = Vec::with_capacity(1024);
        match buf_stdin.read_until(b'\n', &mut buf).await {
            Ok(_) => {
                write_half.write_all(buf).await.0?;
            }
            Err(err) => {
                eprintln!("{}", err);
                break;
            }
        }
    }

    handle.await;

    Ok(())
}
