use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    //set
    client.set("apple", "200".into()).await?;
    client.set("orange", "100".into()).await?;

    //get
    let apple = client.get("apple").await?.unwrap();
    println!("apple: {:?}", String::from_utf8(apple.to_vec()));
    Ok(())
}
