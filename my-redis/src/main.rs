use mini_redis::{Connection, Frame};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    for (stream, _) in listener.accept().await {
        println!("Connection established");
        handler_connection(stream).await;
    }

    Ok(())
}

async fn handler_connection(stream: TcpStream) {
    let mut connection = Connection::new(stream);
    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("get: {:?}", frame);

        //repond
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
