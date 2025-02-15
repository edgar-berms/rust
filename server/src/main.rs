use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use serde_json::json;
use rand::Rng;

mod model;
use model::{JoinTeam, Message, RegisterTeam, ViewTeam};

mod maze;
use maze::Maze;

use common::encode_decode_maze::encode_maze;

#[derive(Debug)]
struct Team {
    name: String,
    access_code: String,
    players: Vec<String>,
    max_players: u8,
    created_at: Instant,
    ready: bool,
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8778")?;
    println!("🚀 Serveur en écoute sur localhost:8778");
    println!("🚀 En attente d'équipe");

    let teams = Arc::new(Mutex::new(HashMap::<String, Team>::new()));

    let maze = Arc::new(Mutex::new(Maze::new(10, 10)));
    maze.lock().unwrap().place_exit();

    let teams_clone = Arc::clone(&teams);
    let maze_clone = Arc::clone(&maze);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(10));
            let mut teams = teams_clone.lock().unwrap();
            teams.retain(|_, team| {
                let elapsed = team.created_at.elapsed().as_secs();
                if elapsed < 300 {
                    true
                } else {
                    println!("🕒 Timeout: Suppression de l'équipe {}", team.name);
                    false
                }
            });
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let teams = Arc::clone(&teams);
                let maze = Arc::clone(&maze);
                thread::spawn(move || {
                    handle_client(stream, teams, maze);
                });
            }
            Err(e) => eprintln!("❌ Erreur de connexion: {}", e),
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream, teams: Arc<Mutex<HashMap<String, Team>>>, maze: Arc<Mutex<Maze>>) {
    let client_addr = stream.peer_addr().unwrap();
    println!("✅ Nouvelle connexion acceptée depuis {}", client_addr);

    let mut raw_data = String::new();
    if let Err(e) = stream.read_to_string(&mut raw_data) {
        eprintln!("❌ Erreur de lecture du message: {}", e);
        return;
    }

    let cleaned_data = raw_data.trim_matches(|c: char| c == '\'' || c.is_whitespace());
    println!("📥 JSON reçu: {:?}", cleaned_data);

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

    let response = match message {
        Message::RegisterTeam(data) => register_team(data, &teams),
        Message::JoinTeam(data) => join_team(data, &teams),
        Message::ViewTeam(data) => view_team(data, &teams),
        Message::GetMaze => get_maze(&maze),
        Message::SetTeamReady(data) => set_team_ready(data, &teams),
        Message::StartGame(data) => start_game(data, &teams),
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
        max_players: data.player_count,
        created_at: Instant::now(),
        ready: false,
    });

    json!({
        "status": "OK",
        "message": "Équipe enregistrée",
        "access_code": access_code,
        "remaining_spots": data.player_count
    }).to_string()
}

fn join_team(data: JoinTeam, teams: &Arc<Mutex<HashMap<String, Team>>>) -> String {
    let mut teams = teams.lock().unwrap();
    for team in teams.values_mut() {
        if team.access_code == data.access_code {
            if team.ready {
                return json!({"status": "ERROR", "message": "L'équipe est déjà prête"}).to_string();
            }
            if team.players.len() as u8 >= team.max_players {
                return json!({
                    "status": "ERROR",
                    "message": "L'équipe est complète"
                }).to_string();
            }
            if team.players.contains(&data.player_name) {
                return json!({
                    "status": "ERROR",
                    "message": "Nom de joueur déjà utilisé"
                }).to_string();
            }
            team.players.push(data.player_name.clone());
            let remaining_spots = team.max_players - team.players.len() as u8;
            return json!({
                "status": "OK",
                "message": "Joueur ajouté",
                "remaining_spots": remaining_spots
            }).to_string();
        }
    }

    json!({
        "status": "ERROR",
        "message": "Code d'accès invalide"
    }).to_string()
}

fn view_team(data: ViewTeam, teams: &Arc<Mutex<HashMap<String, Team>>>) -> String {
    let teams = teams.lock().unwrap();
    if let Some(team) = teams.get(&data.team_name) {
        return json!({
            "status": "OK",
            "players": team.players,
            "ready": team.ready
        }).to_string();
    }
    json!({"status": "ERROR", "message": "Équipe introuvable"}).to_string()
}

fn get_maze(maze: &Arc<Mutex<Maze>>) -> String {
    let maze = maze.lock().unwrap();

    maze.display();
    
    json!({
        "status": "OK",
        "maze": maze.to_string()
    }).to_string()
}

fn start_game(data: ViewTeam, teams: &Arc<Mutex<HashMap<String, Team>>>) -> String {
    let mut teams = teams.lock().unwrap();
    if let Some(team) = teams.get_mut(&data.team_name) {
        if team.ready {
            
            let mut maze = Maze::new(10, 10);
            maze.place_exit();
            let maze_str = maze.to_string();

            let encoded_maze = encode_maze(&maze_str);

            json!({
                "status": "OK",
                "message": "La partie démarre !",
                "encoded_maze": encoded_maze
            }).to_string()
        } else {
            json!({
                "status": "ERROR",
                "message": "L'équipe n'est pas encore prête"
            }).to_string()
        }
    } else {
        json!({
            "status": "ERROR",
            "message": "Équipe introuvable"
        }).to_string()
    }
}

fn set_team_ready(data: ViewTeam, teams: &Arc<Mutex<HashMap<String, Team>>>) -> String {
    let mut teams = teams.lock().unwrap();
    if let Some(team) = teams.get_mut(&data.team_name) {
        if team.players.len() == team.max_players as usize {
            team.ready = true;
            json!({
                "status": "OK",
                "message": "L'équipe est prête pour la partie"
            }).to_string()
        } else {
            json!({
                "status": "ERROR",
                "message": "L'équipe n'a pas encore assez de joueurs"
            }).to_string()
        }
    } else {
        json!({
            "status": "ERROR",
            "message": "Équipe introuvable"
        }).to_string()
    }
}
