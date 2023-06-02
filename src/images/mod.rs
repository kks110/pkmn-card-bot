use std::io::{ErrorKind, Read, Write};
use reqwest::Client;
use futures::StreamExt;
use crate::Error;

const VALID_PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
pub struct Png {
    pub file: File,
}

use std::{
    fs::File,
    path::Path,
};

pub async fn download_image(file_name: &str, url: &str) -> Result<String, Error> {
    let image_root = std::env::var("IMAGE_PATH").expect("Missing image path");

    let path = format!("{}{}", image_root, file_name);
    let mut file;

    if !Path::new(&path).exists() {
        println!("File doesnt exist. Creating");
        file = File::create(&path).or(Err(format!("Failed to create file '{}'", &path)))?;
        download(url, &file).await?;
    }
    while Png::open(&path).is_err() {
        println!("File has an error, trying again");
        std::fs::remove_file(&path)?;
        file = File::create(&path).or(Err(format!("Failed to create file '{}'", &path)))?;
        download(url, &file).await?;
    }

    Ok(path)
}

impl Png {
    fn open(path: &str) -> Result<Png, std::io::Error> {
        let mut file = match File::open(path) {
            Err(e) => return Err(e),
            Ok(f) => f
        };

        if Png::is_valid_signature(&mut file) {
            Ok(Png{file})
        } else {
            Err(std::io::Error::new(ErrorKind::Other, "File does not have a valid png signature."))
        }
    }

    fn is_valid_signature(file: &mut File) -> bool {
        let mut buffer = [0u8; 8];
        let size = file.read(&mut buffer).unwrap();

        if size < 8 {
            false
        } else {
            buffer == VALID_PNG_SIGNATURE
        }
    }
}

async fn download(url: &str, mut file: &File) -> Result<(), Error> {
    let client = Client::new();
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", url)))?;

    let mut stream = res.bytes_stream();

    println!("Commencing transfer");
    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file".to_string()))?;
        file.write_all(&chunk)
            .or(Err("Error while writing to file".to_string()))?;
    }

    println!("Download complete");
    Ok(())
}
