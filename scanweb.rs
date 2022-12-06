use std::fs;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

fn main() -> io::Result<()> {
    if std::env::args().len() != 3 {
        eprintln!("Usage: {} <url> <dictionary_file>", std::env::args().next().unwrap());
        std::process::exit(1);
    }

    // Ouvrez le fichier dictionnaire
    let dict_filename = std::env::args().nth(2).unwrap();
    let dict_file = fs::File::open(dict_filename)?;
    let mut dict_reader = io::BufReader::new(dict_file);

    // Définissez l'URL cible
    let url = std::env::args().nth(1).unwrap();

    // Initialisez la structure de stockage des réponses HTTP
    let mut response = Vec::new();

    // Parcourez le fichier dictionnaire
    let mut directory = String::new();
    while dict_reader.read_line(&mut directory)? > 0 {
        let directory = directory.trim();

        // Préparez la requête HTTP GET
        let request = format!(
            "GET /{} HTTP/1.1\r\nHost: {}\r\n\r\n",
            directory,
            url
        );

        // Effectuez la requête HTTP
        let mut stream = TcpStream::connect(url)?;
        stream.write_all(request.as_bytes())?;
        response.clear();
        stream.read_to_end(&mut response)?;

        // Analysez la réponse HTTP
        let response_str = str::from_utf8(&response).unwrap();
        if response_str.starts_with("HTTP/1.1 200 OK") {
            println!("Directory found: /{}", directory);
        }

        directory.clear();
    }

    Ok(())
}
