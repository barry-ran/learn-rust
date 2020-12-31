use reqwest;
use std::fs;
use std::thread;
use tokio::task;
use tokio_compat_02::FutureExt;
/*
test server:
cargo install simple-http-server
simple-http-server -i -u .

test client:
curl -v -F a=1 -F upload=@chunk_1.webm http://0.0.0.0:8000/
*/

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let data = fs::read("/Users/barry/chunk_1.webm").unwrap();
    let part = reqwest::multipart::Part::bytes(data).file_name("chunk_1.webm");    
    let form = reqwest::multipart::Form::new()
        .text("foo", "bar")
        .part("part_stream", part);        
    let url = format!("http://0.0.0.0:8000/");

    let client = reqwest::Client::new();

    println!("begin request");

    let res = client
        .post(&url)
        .multipart(form)
        .send()
        .compat()
        .await
        .expect("Failed to post multipart");

    println!("end request");
}
