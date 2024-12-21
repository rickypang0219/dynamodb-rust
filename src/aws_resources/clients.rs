use aws_sdk_dynamodb as dynamodb;
use aws_sdk_ssm as ssm;

pub async fn get_config() -> aws_config::SdkConfig {
    aws_config::load_from_env().await
}

pub async fn get_ddb_client() -> Result<dynamodb::Client, dynamodb::Error> {
    let config = get_config().await;
    let ddb_client = dynamodb::Client::new(&config);
    Ok(ddb_client)
}

pub async fn get_ssm_client() -> Result<ssm::Client, ssm::Error> {
    let config = get_config().await;
    let ssm_client = ssm::Client::new(&config);
    Ok(ssm_client)
}
