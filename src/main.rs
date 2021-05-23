
use std::env;
use tokio::fs::{OpenOptions};
use tokio::io::{self, BufWriter, BufReader, AsyncBufReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = match args.get(1) {
        Some(name) => name,
        None => {
            eprintln!("You must specify a filename to output to!");
            return Ok(());
        }
    };

    let append = args.iter().any(|arg| arg == "-a");

    // Open the output file.
    let file = match OpenOptions::new()
        .write(true)
        .append(append)
        .create(true)
        .open(filename)
        .await {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Couldn't open file: {}", err);
                return Ok(());
            }
        };

    let mut file_buffer = BufWriter::new(file);

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();
    while let Ok(len) = reader.read_line(&mut line).await {
        if len == 0 {
            break;
        }

        println!("{}", line);
        file_buffer.write_all(line.as_bytes()).await.unwrap();

        line.clear();
    }

    file_buffer.flush().await.unwrap();
    Ok(())
}
