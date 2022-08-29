use lambda_runtime::{service_fn, LambdaEvent, Error};
use reqwest::Client;
use serde_json::{json, Value};
use xmltojson::{to_json};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(_event: LambdaEvent<Value>) -> Result<Value, Error> {
    let client = Client::new();

    Ok(json!({}))
}
