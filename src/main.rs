use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Photo {
    #[serde(rename="albumId")]
    album_id: u32,
    id: Option<u32>,
    title: String,
    url: String,
    #[serde(rename="thumbnailUrl")]
    thumbnail_url: String,
}

#[tokio::main]

async fn main() -> Result<(), reqwest::Error> {
    // let photos: Vec<Photo> = reqwest::Client::new()
    //     .get("https://jsonplaceholder.typicode.com/photos?albumId=1")
    //     .send().await?.json().await?;
    // println!("{:#?}", todos);

    let new_photo = Photo {
        album_id: 1,
        id: Some(234),
        title: "you know what time it is".to_owned(),
        url: "https://imgur.com/iykyk.jpg".to_owned(),
        thumbnail_url: "https://imgur.com/iykyk-thumb.jpg".to_owned(),
    };
    let new_photo: serde_json::Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/photos")
        // .json(&new_todo)
        .json(&serde_json::json!({
            "albumId": 1,
            "id": 234,
            "title": "you know what time it is",
            "url": "https://imgur.com/iykyk.jpg",
            "thumbnail_url": "https://imgur.com/iykyk-thumb.jpg"
        }))
        .send().await?.json().await?;
    println!("{:#?}", new_photo);
    Ok(())
}

