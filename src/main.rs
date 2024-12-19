pub mod bookticker_stream;
use bookticker_stream::bookticker::BookTickerStream;

pub mod dynamo;
use dynamo::client::get_client;

#[tokio::main]
async fn main() -> Result<(), aws_sdk_dynamodb::Error> {
    let ddb_client = get_client().await?;

    let bookticker_stream = BookTickerStream::new();
    let listener_task = {
        let bookticker_stream_clone = bookticker_stream.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = bookticker_stream_clone
                    .listen_coins_book_prices(&ddb_client)
                    .await
                {
                    eprintln!("Error listening to WebSocket: {:?}", e);
                    continue;
                }
            }
        })
    };

    let printer_task = {
        let bookticker_stream_clone = bookticker_stream.clone();
        tokio::spawn(async move {
            loop {
                bookticker_stream_clone.show_bookticker().await;
            }
        })
    };
    let _ = tokio::try_join!(listener_task, printer_task);
    Ok(())
}
