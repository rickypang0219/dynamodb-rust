use aws_sdk_ssm::config::http::HttpResponse;
use aws_sdk_ssm::error::SdkError;
use aws_sdk_ssm::operation::get_parameter::{GetParameterError, GetParameterOutput};
use tracing::info;

async fn request_param(
    ssm_client: &aws_sdk_ssm::Client,
    param_name: String,
) -> Result<GetParameterOutput, SdkError<GetParameterError, HttpResponse>> {
    let param = ssm_client
        .get_parameter()
        .name(param_name)
        .with_decryption(true)
        .send()
        .await;
    match param {
        Ok(data) => Ok(data),
        Err(e) => {
            info!("Unable to get parameter!");
            Err(e)
        }
    }
}

pub async fn get_param_value(
    ssm_client: &aws_sdk_ssm::Client,
    param_name: String,
) -> Result<Option<String>, SdkError<GetParameterError, HttpResponse>> {
    let param_info = request_param(ssm_client, param_name).await?;
    if let Some(parameter) = param_info.parameter() {
        if let Some(value) = parameter.value() {
            return Ok(Some(value.to_string()));
        }
    }
    Ok(None)
}
