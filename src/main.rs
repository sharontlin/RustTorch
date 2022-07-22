use rusttorch::util::config::CONFIG;

use reqwest;
use tch;
use tch::Tensor;
use tch::vision::imagenet;
use std::fs::File;
use std::io::{prelude::*, Cursor};
use tch::Kind::*;
use core::convert::From;

async fn get_remote_model(username: &str, path_to_pass: &str, url: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let username = username.to_string();
    let mut password = String::new();
    let mut _file = File::open(path_to_pass)?.read_to_string(&mut password)?;
    let password = password.trim_end_matches(&['\n'][..]).to_string();

    // Download model binary 
    let result = client
        .get(url)
        .basic_auth(&username, Some(&password))
        .send()
        .await
        .expect("Unable to send request")
        .bytes()
        .await
        .expect("Unable to download file");

    // Save model binary into `model.pt`
    let mut bin = std::fs::File::create("model.pt")?;
    let mut model =  Cursor::new(result);
    std::io::copy(&mut model, &mut bin)?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let username = &CONFIG.username;
    let path_to_pass = &CONFIG.path_to_pass;
    let url = &CONFIG.url;
    let image_name = &CONFIG.image_name;

    // Fetch remote model
    get_remote_model(username, path_to_pass, url).await.expect("Unable to save model");
    
    // Unpack the model binary
    let model = tch::CModule::load("model.pt").expect("Unable to load model");

    // Load image (note: if image is already in grayscale then R==G==B)
    let image = imagenet::load_image(format!("data/{}", image_name)).expect("Unable to load image");
    let image = image.get(0);
    let image = image.reshape(&[1, 1, 1200, 1200]);

    // Apply model to image, output feature maps)
    let output = model.forward_ts(&[image])?.softmax(-1, Float).get(0);

    for i in 0..16 {
        // Normalize values 
        let row = output.get(i);
        let row = row * 256 * 1000;
        let row = row.floor().clamp(0, 255);

        // Compute threshold 
        let max = row.max();
        let min = row.min();
        let threshold = &min - ((&min-&max)/4);
        let threshold: i64 = From::from(threshold);

        // Binary threshold 
        let row = Tensor::where_scalar(&row.greater(threshold), 255, 0);
        let row = row.reshape(&[1, 1200, 1200]);
        
        // Save thresholded image 
        tch::vision::image::save(&row, format!("image_{index}.png", index=i))?;
    }

    Ok(())
}