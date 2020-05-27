use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct CreateGistFilePayload {
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct CreateGistPayload {
    pub description: String,
    pub public: bool,
    pub files: std::collections::HashMap<String, CreateGistFilePayload>,
}

#[derive(Deserialize, Debug)]
pub struct CreateGistResponsePayload {
    pub url: String,
    pub html_url: String,
}

pub fn create_gist(files: Vec<String>, is_public: bool, description: String) {
    println!("create_gist got called");
    println!("files: {:?}", files);
    println!("is_public: {:?}", is_public);
    println!("description: {:?}", description);
    // let mut file = File::open("test.gist")?;
    // let mut gist_data = String::new();
    // file.read_to_string(&mut gist_data)?;

    // let mut files = std::collections::HashMap::new();
    // files.insert(
    //     String::from("test.gist"),
    //     CreateGistFilePayload { content: gist_data },
    // );

    // let payload = CreateGistPayload {
    //     description: String::from("test gist"),
    //     public: true,
    //     files: files,
    // };

    // let url: &str = "https://api.github.com/gists";
    // let client = reqwest::Client::new();
    // let token = "56122920292a664576ebd5ded0e381ba88dc7ea0";
    // let req = client
    //     .post(url)
    //     .json(&payload)
    //     .header("user-agent", "gst")
    //     .header("authorization", format!("token {}", token));

    // let resp = req.send().await?;

    // println!("{:?}", resp.status());
    // println!("{:?}", resp.json::<CreateGistResponsePayload>().await?);
}
