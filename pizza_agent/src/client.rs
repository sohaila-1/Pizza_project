use std::net::TcpStream;
use std::io::{Read, Write};

use ciborium::{ser::into_writer, de::from_reader};

mod protocol;
use protocol::{Request, Response};

use std::time::Duration;
use std::thread::sleep;

    let mut stream = loop {
        match TcpStream::connect("127.0.0.1:9000") {
            Ok(s) => {
                println!("✅ Connecté !");
                break s;
            }
            Err(_) => {
                println!("🔁 Retry connexion...");
                sleep(Duration::from_secs(1));
            }
        }
    };

// timeout lecture
    stream.set_read_timeout(Some(Duration::from_secs(5))).ok();
    .unwrap();

    println!("✅ Connecté !");

    let request = Request::Order {
        recipe_name: "Margherita".to_string(),
    };

    let mut buffer = Vec::new();
    into_writer(&request, &mut buffer).unwrap();

    // envoyer taille + data
    let len = (buffer.len() as u32).to_be_bytes();
    stream.write_all(&len).unwrap();
    stream.write_all(&buffer).unwrap();

    println!("📤 Requête envoyée");

    // lire réponse
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).unwrap();

    let len = u32::from_be_bytes(len_buf);

    let mut data = vec![0u8; len as usize];
    stream.read_exact(&mut data).unwrap();

    match from_reader::<Response, _>(&data[..]) {
        Ok(res) => println!("🍕 Réponse: {:#?}", res),
        Err(e) => println!("❌ Erreur: {:?}", e),
    }
}