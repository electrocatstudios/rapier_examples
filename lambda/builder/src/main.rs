use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use renderer::{render,file_loader::LevelDescriptor,Context};
use serde_json::json;
use aws_lambda_events::s3::object_lambda::{GetObjectContext, S3ObjectLambdaEvent};
use aws_sdk_s3::Client as S3Client;
use s3::{GetFile, SendFile};

mod s3;

use std::io::{Write,BufWriter};
use std::fs::File;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<String>, Error> {
    // Extract some useful information from the request
    let level_descriptor_res: Result<LevelDescriptor, _> = match event.body() {
        Body::Text(body) => serde_json::from_str(body),
        Body::Binary(body) => serde_json::from_slice(body),
        _ => {
            return Ok(Response::builder()
                .status(400)
                .header("Content-Type", "application/json")
                .body(json!({"message": "Empty Body"}).to_string())
                .expect("failed to render response"));
        }
    };

    let level_descriptor: LevelDescriptor = match level_descriptor_res {
        Ok(ld) => ld,
        Err(err) => {
            return Ok(Response::builder()
                .status(400)
                .header("Content-Type", "application/json")
                .body(json!({"message": err.to_string()}).to_string())
                .expect("failed to render response"));
        } 
    };
    
    {
        // Write input json to tmp folder
        let file = File::create("/tmp/input.json")?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &level_descriptor)?;
        writer.flush()?;
    }


    let guid = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("guid"))
        .unwrap_or("");

    if guid == "" {
        return Ok(Response::builder()
                .status(400)
                .header("Content-Type", "application/json")
                .body(json!({"message": "Missing guid"}).to_string())
                .expect("failed to render response"));
    }

    let bucket_name = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("s3"))
        .unwrap_or("<UNKNOWN_BUCKET>");

    let shared_config = aws_config::load_from_env().await;
    let client = S3Client::new(&shared_config);
    let client_ref = &client;

    let ctx: Context = Context{
        frame_width: 1920,
        frame_height: 1080,
        max_frames: 1000,
        debug: true
    };
    match render(level_descriptor, "/tmp/output.mp4".to_string(), ctx) {
        Ok(_) => {},
        Err(err) => panic!("{}", err)
    }
    
    let message = format!("Finished creating video");
    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
