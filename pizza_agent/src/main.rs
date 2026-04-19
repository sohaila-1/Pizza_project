use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::time::Duration;
use std::fs;

use ciborium::{de::from_reader, ser::into_writer};

mod handler;
mod protocol;
use protocol::{Request, Response};

fn handle_client(mut stream: TcpStream, recipes: &Vec<String>) {
    println!("📩 Client connecté");

    // timeout
    stream.set_read_timeout(Some(Duration::from_secs(5))).ok();

    // Lire taille
    let mut len_buf = [0u8; 4];
    if let Err(e) = stream.read_exact(&mut len_buf) {
        println!("❌ Erreur lecture taille: {:?}", e);
        return;
    }

    let len = u32::from_be_bytes(len_buf);

    // Lire data
    let mut data = vec![0u8; len as usize];
    if let Err(e) = stream.read_exact(&mut data) {
        println!("❌ Erreur lecture data: {:?}", e);
        return;
    }

    // Décoder
    match from_reader::<Request, _>(&data[..]) {
        Ok(req) => {
            println!("📨 Requête reçue: {:?}", req);

            let result = handler::process_request(req, recipes);

            let response = Response::ProductionError {
                order_id: None,
                error: result,
            };

            let mut buffer = Vec::new();

            if let Err(e) = into_writer(&response, &mut buffer) {
                println!("❌ Erreur sérialisation: {:?}", e);
                return;
            }

            let len = (buffer.len() as u32).to_be_bytes();

            if let Err(e) = stream.write_all(&len) {
                println!("❌ Erreur envoi taille: {:?}", e);
                return;
            }

            if let Err(e) = stream.write_all(&buffer) {
                println!("❌ Erreur envoi data: {:?}", e);
                return;
            }

            println!("📤 Réponse envoyée");
        }
        Err(e) => {
            println!("❌ Erreur decode: {:?}", e);
        }
    }
}

fn main() {
    println!("🔗 Connexion au réseau pizza_factory...");

    // 🔥 1. Lire fichier
    let recipes_content = fs::read_to_string(
        "/Users/sohaila/pizza-project/pizza_factory/recipes/examples.recipes"
    ).expect("❌ Impossible de lire le fichier recipes");

    println!("📄 Recipes loaded:\n{}", recipes_content);

    // 🔥 2. Parser
    let recipes_list = parse_recipes(&recipes_content);

    println!("🍕 Recettes disponibles: {:?}", recipes_list);

    // Connexion test
    match TcpStream::connect("127.0.0.1:8000") {
        Ok(_) => println!("✅ Connecté au réseau !"),
        Err(e) => println!("⚠️ Pas connecté (normal): {:?}", e),
    }

    let listener = TcpListener::bind("127.0.0.1:9000")
        .expect("❌ Impossible de lancer le serveur");

    println!("🚀 Agent lancé sur 127.0.0.1:9000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("📩 Nouvelle connexion");
                handle_client(stream, &recipes_list);
            }
            Err(e) => println!("❌ Erreur connexion: {:?}", e),
        }
    }
}

fn parse_recipes(content: &str) -> Vec<String> {
    content
        .lines()
        .filter(|line| line.trim().ends_with("="))
        .map(|line| line.split('=').next().unwrap().trim().to_string())
        .collect()
}