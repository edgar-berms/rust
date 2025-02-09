use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use serde_json::json;
use rand::Rng;

mod model;
use model::{Message, RegisterTeam, JoinTeam};

#[derive(Debug)]
struct Team {
    name: String,
    access_code: String,
    players: Vec<String>,
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8778")?;
    println!("🚀 Serveur en écoute sur localhost:8778");

    let teams = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let teams = Arc::clone(&teams);
                std::thread::spawn(move || {
                    handle_client(stream, teams);
                });
            }
            Err(e) => eprintln!("❌ Erreur de connexion: {}", e),
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream, teams: Arc<Mutex<HashMap<String, Team>>>) {
    let client_addr = stream.peer_addr().unwrap();
    println!("✅ Nouvelle connexion acceptée depuis {}", client_addr);

    let mut raw_data = String::new();
    if let Err(e) = stream.read_to_string(&mut raw_data) {
        eprintln!("❌ Erreur de lecture du message: {}", e);
        return;
    }

    // Nettoyage du JSON
    let cleaned_data = raw_data.trim_matches(|c: char| c == '\'' || c.is_whitespace());
    println!("📥 JSON reçu: {:?}", cleaned_data);

    // Tentative de parsing du JSON
    let message: Message = match serde_json::from_str(cleaned_data) {
        Ok(msg) => msg,
        Err(e) => {
            eprintln!("❌ Erreur de parsing JSON: {}", e);
            let response = json!({
                "status": "ERROR",
                "message": "Format JSON invalide"
            });
            writeln!(stream, "{}", response).unwrap();
            return;
        }
    };

    // Traitement du message reçu
    let response = match message {
        Message::RegisterTeam(data) => register_team(data, &teams),
        Message::JoinTeam(data) => join_team(data, &teams),
    };

    writeln!(stream, "{}", response).unwrap();
    println!("🚀 Fermeture de la connexion avec {}", client_addr);
}

fn register_team(data: RegisterTeam, teams: &Arc<Mutex<HashMap<String, Team>>>) -> String {
    let mut teams = teams.lock().unwrap();
    if teams.contains_key(&data.team_name) {
        return json!({
            "status": "ERROR",
            "message": "Nom d'équipe déjà pris"
        }).to_string();
    }

    let access_code: String = rand::thread_rng().gen_range(1000..9999).to_string();
    teams.insert(data.team_name.clone(), Team {
        name: data.team_name.clone(),
        access_code: access_code.clone(),
        players: Vec::new(),
    });

    json!({
        "status": "OK",
        "message": "Équipe enregistrée",
        "access_code": access_code
    }).to_string()
}

fn join_team(data: JoinTeam, teams: &Arc<Mutex<HashMap<String, Team>>>) -> String {
    let mut teams = teams.lock().unwrap();
    for team in teams.values_mut() {
        if team.access_code == data.access_code {
            if team.players.contains(&data.player_name) {
                return json!({
                    "status": "ERROR",
                    "message": "Nom de joueur déjà utilisé"
                }).to_string();
            }
            team.players.push(data.player_name.clone());
            return json!({
                "status": "OK",
                "message": "Joueur ajouté"
            }).to_string();
        }
    }

    json!({
        "status": "ERROR",
        "message": "Code d'accès invalide"
    }).to_string()
}
