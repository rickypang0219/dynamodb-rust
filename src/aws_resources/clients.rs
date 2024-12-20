use aws_sdk_dynamodb as dynamodb;

pub async fn get_client() -> Result<dynamodb::Client, aws_sdk_dynamodb::Error> {
    let config = aws_config::load_from_env().await;
    let ddb_client = dynamodb::Client::new(&config);
    Ok(ddb_client)
}
