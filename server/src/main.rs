use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")] // Permet de différencier les types de messages
enum Message {
    Connect { team_name: String },
    Register { team_code: String },
    Move { direction: String },
    Challenge { answer: i32 },
}

#[derive(Debug, Serialize)]
struct ServerResponse {
    status: String,
    message: String,
}

fn handle_client(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    println!("✅ Nouvelle connexion acceptée depuis {}", peer_addr);

    let mut reader = BufReader::new(stream.try_clone().unwrap()); // 🔥 Clone du stream
    let mut writer = &mut stream; // ✅ Permet un emprunt mutable séparé pour l'écriture

    for line in reader.lines() {
        match line {
            Ok(msg) => {
                println!("📥 Reçu brut: {:?}", msg);

                // ✅ Nettoyage du message JSON
                let cleaned_msg = msg.trim_matches(|c| c == '\'' || c == ' '); 
                println!("🔍 JSON nettoyé: {:?}", cleaned_msg);

                // ✅ Parser le JSON proprement
                let parsed: Result<Message, serde_json::Error> = serde_json::from_str(cleaned_msg);
                match parsed {
                    Ok(message) => {
                        println!("✅ Message reçu: {:?}", message);

                        // Réponse au client
                        let response = ServerResponse {
                            status: "OK".to_string(),
                            message: "Message bien reçu".to_string(),
                        };
                        let response_json = serde_json::to_string(&response).unwrap();

                        if let Err(e) = writeln!(writer, "{}\n", response_json) {
                            println!("⚠️ Erreur lors de l'envoi de la réponse: {}", e);
                        }
                    }
                    Err(e) => {
                        println!("❌ Erreur de parsing JSON: {}", e);
                        let error_response = ServerResponse {
                            status: "ERROR".to_string(),
                            message: "Format JSON invalide".to_string(),
                        };
                        let error_json = serde_json::to_string(&error_response).unwrap();
                        writeln!(writer, "{}\n", error_json).unwrap();
                    }
                }
            }
            Err(e) => {
                println!("⚠️ Erreur lors de la lecture du message: {}", e);
            }
        }
    }

    println!("🚀 Fermeture de la connexion avec {}", peer_addr);
}



fn main() {
    let listener = TcpListener::bind("127.0.0.1:8778").expect("🔥 Impossible de démarrer le serveur !");
    println!("🚀 Serveur en écoute sur localhost:8778");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("⚠️ Erreur de connexion : {}", e);
            }
        }
    }
}
