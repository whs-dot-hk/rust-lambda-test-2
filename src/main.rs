use aws_lambda_events::event::cloudwatch_logs::CloudwatchLogsEvent;use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use hyper_tls::HttpsConnector;
use hyper::Client;


/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<CloudwatchLogsEvent>) -> Result<(), Error> {
    // Extract some useful information from the request
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get("https://hyper.rs".parse()?).await?;
    assert_eq!(res.status(), 200);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
