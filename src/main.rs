use gst;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    gst::create_gist(
        vec![String::from("test.gist")],
        true,
        String::from("Test 1234"),
    );

    Ok(())
}
