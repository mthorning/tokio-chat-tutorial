use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:9002").await.unwrap();

    let (tx, mut _rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        // need to clone because tx cannot be moved into the loop
        let tx = tx.clone();

        // this is strange, can't move rx into the loop below but we don't clone the rx from the
        // channel, instead we can get an rx from the cloned tx by using subscribe:
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            // can split read part from write part so that BufReader can take ownership of read part
            // and write part can be used within the loop:
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result =  reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            // empty line should disconnect the session
                            break;
                        }

                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();

                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
