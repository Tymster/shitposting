use tide::prelude::*;
mod prnt;
use tide::Request;
use tide::StatusCode;
#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("handling response");
    let mut app = tide::new();
    app.at("/image").get(send_image);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
async fn send_image(req: Request<()>) -> tide::Result {
    let client = reqwest::blocking::Client::new();
    let mut contents: Vec<u8> = vec![];
    client
        .get(prnt::random_url())
        .send()
        .unwrap()
        .copy_to(&mut contents)
        .unwrap();
    let mut resp = tide::Response::new(StatusCode::Ok);
    resp.set_body(tide::Body::from_bytes(contents.to_vec()));
    resp.set_content_type("image/jpeg");
    Ok(resp)
}
