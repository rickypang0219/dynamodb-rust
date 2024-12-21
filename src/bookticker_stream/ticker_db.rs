use crate::bookticker_stream::bookticker::BookTicker;
use aws_sdk_dynamodb::types::AttributeValue;
use tracing::info;

pub async fn put_ticker_to_db(
    client: &aws_sdk_dynamodb::Client,
    item: BookTicker,
) -> Result<(), aws_sdk_dynamodb::Error> {
    let event = AttributeValue::S(format!("{}#{}", item.symbol, item.event));
    let symbol = AttributeValue::S(item.symbol);
    let best_bid = AttributeValue::N(item.best_bid);
    let bid_qty = AttributeValue::N(item.bbid_qty);
    let best_ask = AttributeValue::N(item.best_ask);
    let ask_qty = AttributeValue::N(item.ask_qty);
    let event_time = AttributeValue::S(item.event_time.to_string());

    let request = client
        .put_item()
        .table_name("TestTickerTable")
        .item("PK", event)
        .item("SK", event_time)
        .item("symbol", symbol)
        .item("best_bid", best_bid)
        .item("bid_qty", bid_qty)
        .item("best_ask", best_ask)
        .item("ask_qty", ask_qty);

    info!("Executing request to DynamoDB");
    request.send().await?;
    info!("Successfully uploaded BookTicker to TickerTable",);
    Ok(())
}
