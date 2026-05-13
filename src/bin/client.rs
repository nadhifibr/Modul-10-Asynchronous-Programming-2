use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin_lines = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            line = stdin_lines.next_line() => {
                let line = line?.unwrap_or_default();
                if line.is_empty() { continue; }
                ws_stream.send(Message::text(line)).await?;
            }
            msg = ws_stream.next() => {
                if let Some(Ok(msg)) = msg {
                    if let Some(text) = msg.as_text() {
                        println!("From server: {text}");
                    }
                } else {
                    break;
                }
            }
        }
    }
    Ok(())
}