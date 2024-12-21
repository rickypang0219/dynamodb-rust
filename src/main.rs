pub mod bookticker_stream;
use bookticker_stream::bookticker::BookTickerStream;
use tracing::{info, Level};
use tracing_subscriber;

pub mod async_binance;
pub mod aws_resources;
pub mod order_stream;
use async_binance::client_async::AsyncBinanceClient;
use aws_resources::clients::get_ssm_client;
use aws_resources::ssm_params::get_param_value;
use order_stream::order_update::UserDataStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ssm_client = get_ssm_client().await?;
    let binance_api_key = get_param_value(&ssm_client, "binance-api-key".to_string()).await?;
    let binance_secret_key = get_param_value(&ssm_client, "binance-secret-key".to_string()).await?;
    println!("{:?} {:?}", &binance_api_key, &binance_secret_key);

    let binance_future_client = AsyncBinanceClient::new(
        binance_api_key,
        binance_secret_key,
        "https://fapi.binance.com/fapi/v1/".to_string(),
        Some(30),
    );
    let listen_key: String = binance_future_client.get_listen_key().await?;
    println!("{:?}", &listen_key);

    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let bookticker_stream = BookTickerStream::new();
    let listener_task = {
        let bookticker_stream_clone = bookticker_stream.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = bookticker_stream_clone.listen_coins_book_prices().await {
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

    let user_data_stream = UserDataStream {
        listen_key: listen_key.clone(),
    };
    let user_data_listener_task = {
        let user_data_stream_clone = user_data_stream.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = user_data_stream_clone.listen_user_data().await {
                    info!("Error listening to user data stream: {:?}", e);
                    continue;
                }
            }
        })
    };

    let keep_listen_key_alive_task = tokio::spawn(async move {
        let interval = tokio::time::Duration::from_secs(1800);
        loop {
            if let Err(e) = binance_future_client
                .keep_listen_key_alive(&listen_key.clone())
                .await
            {
                info!("Error in keeping ListenKey alive {:?}", e);
                continue;
            }
            tokio::time::sleep(interval).await;
        }
    });

    let _ = tokio::try_join!(
        listener_task,
        printer_task,
        user_data_listener_task,
        keep_listen_key_alive_task
    );
    Ok(())
}
