// https://docs.rs/surge-ping/latest/surge_ping/
// https://github.com/kolapapa/surge-ping/blob/main/examples/multi_ping.rs
// https://tokio.rs/tokio/tutorial/channels

const TIMEOUT: u64 = 1;             // Délai d'attente en secondes avant de considérer un ping comme non répondu

use std::net::IpAddr;
use std::time::Duration;
use chrono::Utc;
use futures::future::join_all;
use rand::random;
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence};
use tokio::sync::mpsc;

// Définition d'un message à passer dans le channel inter-tâches
enum Command {
    Ping {
        addr: String,               // IP de l'hôte pingé
        timestamp: String,          // timestamp de la réponse
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tableau des adresses IPv4 à pinger
    let ips = [
        "192.168.0.7",
        "192.168.0.6",
        "192.168.0.13",
        "192.168.0.19",
        "10.5.0.1",
        "10.5.0.2",
        "10.5.0.3",
        "10.5.0.4",
        "8.8.8.8"
    ];
    // Création du channel inter-tâches de capacité 32
    // tx est le canal d'envoi, rx celui de réception
    let (tx, mut rx) = mpsc::channel(32);
    // tableau des tâches de ping
    let mut tasks = Vec::new();
    // Client IPv4 uniquement
    let client_v4 = Client::new(&Config::default())?;

    // Liste de ping (émetteur du channel)
    // On crée une tâche de ping par adresse IPv4 du tableau valide
    for ip in &ips {
        // analyse de la string IP du tableau des IP
        match ip.parse() {
            // si l'adresse est une IPv4 valide
            Ok(IpAddr::V4(addr)) => {
                // on stocke une tâche dans le tableau des tâches à partir d'un clone du client
                let txc = tx.clone(); // on clone le canal émetteur du channel
                let client_clone = client_v4.clone();
                tasks.push(tokio::spawn( async move {
                    // la tâche attend un résultat du ping et si OK le transmet dans le channel à la tâche logger
                    let ping_res = ping(client_clone, IpAddr::V4(addr)).await; // appel effectif du ping
                    if let Ok(cmd) = ping_res {
                        txc.send(cmd).await.unwrap();   // envoi du message Command::Ping à la tâche logger
                        drop(txc);                      // puis on ferme explicitement l'émetteur
                    }
                }))
            },
            // si l'adresse est une IPv6 valide
            Ok(IpAddr::V6(addr)) => {
                println!("ping IPv6 non activé {}", addr); // message d'information
            },
            // Si l'adresse n'est pas valide -> message d'information
            Err(e) => println!("{} erreur d\'analyse de l\'adresse IP : {}", ip, e),
        }
    }
    drop(tx); // on supprime l'émetteur qui a servi de base pour les clone


    // On crée une tâche de log, elle va recevoir en boucle les messages des tâches de ping via le channel
    // et gérer l'écriture dans le fichier de log
    let logger = tokio::spawn(async move {
        // le récepteur sera automatiquement fermé quand tous les émetteurs auront été drop()
        if !rx.is_closed() {
            while let Some(cmd) = rx.recv().await {
                match cmd {
                    Command::Ping {addr, timestamp} => { log(addr, timestamp).await; }
                }
            }
        }
        drop(rx);
        println!("Liste d'IP entièrement pingée");
    });

    // Démarrage des tâches :
    let start = Utc::now();
    println!("Pinger démarré à {:?}", start);
    logger.await.unwrap();      // on lance la tâche logger
    join_all(tasks).await;      // on rassemble toutes les tâches de ping pour le scheduler et on les démarre
    let now = Utc::now();
    let elapsed = now - start;
    println!("Pinger terminé en {:?} secondes", elapsed.num_seconds());
    Ok(())                      // on renvoie un Result Ok vide
}

// fonction ping : consomme une instance de Client et une IpAddr
// Renvoie un Result : un Command::Ping si Ok, une Err() sinon
 async fn ping(client: Client, addr: IpAddr) -> Result<Command, ()> {
    let payload = [0; 56];
    let mut pinger = client.pinger(addr, PingIdentifier(random())).await;
    pinger.timeout(Duration::from_secs(TIMEOUT));
    match pinger.ping(PingSequence(0), &payload).await {
        Ok((IcmpPacket::V4(packet), _duration)) => {
            let timestamp = "now".to_string();
            let addr = packet.get_source().to_string();
            let command = Command::Ping{addr, timestamp};
            Ok(command)
        },
        Ok((IcmpPacket::V6(_packet), _dur)) => {
            println!("ping IPv6 non activé : {:?}", addr);
            Err(())
        },
        Err(_e) => {
            //println!("{} ping {}", pinger.host, e);
            Err(())
        },
    }
}

async fn log(addr: String, timestamp: String) {
    println!("ip={}, timestamp={}", addr, timestamp);
}