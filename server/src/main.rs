use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
enum Message {
    Connect { team_name: String },
    Register { team_code: String },
    Move { direction: String },
    Challenge { answer: i32 },
}

async fn handle_client(stream: TcpStream) {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader).lines();

    while let Ok(Some(line)) = reader.next_line().await {
        let line = line.trim().trim_matches('\''); // Supprime les espaces et caractères parasites
        println!("Reçu brut: {:?}", line);

        match serde_json::from_str::<Message>(line) {
            Ok(message) => {
                println!("Message reçu: {:?}", message);
                let response = serde_json::to_string(&"Message reçu").unwrap();
                let _ = writer.write_all(response.as_bytes()).await;
            }
            Err(e) => {
                println!("Erreur de parsing JSON: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8778").await.unwrap();
    println!("Serveur en écoute sur localhost:8778");

    while let Ok((stream, _)) = listener.accept().await {
        println!("Nouvelle connexion acceptée");
        tokio::spawn(handle_client(stream));
    }
}
