use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:9002").await.unwrap();

    let (mut socket, _addr) = listener.accept().await.unwrap();

    // can split read part from write part so that BufReader can take ownership of read part
    // and write part can be used within the loop:
    let (reader, mut writer) = socket.split();


    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        let bytes_read = reader.read_line(&mut line).await.unwrap();
        if bytes_read == 0 {
            // empty line should disconnect the session
            break;
        }

        writer.write_all(line.as_bytes()).await.unwrap();
        line.clear();
    }
}
