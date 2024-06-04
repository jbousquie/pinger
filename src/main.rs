// https://docs.rs/surge-ping/latest/surge_ping/
// https://github.com/kolapapa/surge-ping/blob/main/examples/multi_ping.rs
// https://tokio.rs/tokio/tutorial/channels


const IP_FILENAME: &str = "adresses.txt"; //nom du fichier des adresses à pinger
const LOG_FILENAME: &str = "ping.log";   // nom du fichier de log des pings
const SEP: &str = ",";                  // caractère séparateur dans le fichier de log
const TIMEOUT: u64 = 1;             // Délai d'attente en secondes avant de considérer un ping comme non répondu
mod pinged;
use std::process;
use std::net::IpAddr;
use std::time::Duration;
use chrono::Utc;
use futures::future::join_all;
use rand::random;
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence};
use tokio::sync::mpsc;
use crate::pinged::{get_ping_ips, update_data_from_logfile, update_filelog};

/// Définition d'un message à passer dans le channel inter-tâches
enum Command {
    Ping {
        addr: String,               // IP de l'hôte pingé
        timestamp: String,          // timestamp de la réponse
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Récupération de la Hashmap des adresses IPv4 à pinger
    let mut ips = get_ping_ips(IP_FILENAME);
    let nb = ips.len();
    if nb == 0 {
        println!("fichier des adresses à pinger {IP_FILENAME} vide.");
        process::exit(0x0100);
    }
    // Récupération éventuellement des logs précédents depuis le fichier et mise à jour de la HashMap
    ips = update_data_from_logfile(LOG_FILENAME, SEP, ips);

    // Création du channel inter-tâches de capacité 32
    // tx est le canal d'envoi, rx celui de réception
    let (tx, mut rx) = mpsc::channel(32);
    // tableau des tâches de ping
    let mut tasks = Vec::new();
    // Client IPv4 uniquement
    let client_v4 = Client::new(&Config::default())?;

    // Liste de ping (émetteur du channel)
    // On crée une tâche de ping par adresse IPv4 du tableau valide
    for (ip, _ts) in &ips {
        // analyse de la string IP clé de la HashMap
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
    let logger = tokio::spawn(async move  {
        let mut cpt = 0;  // compteur de pings répondus
        // le récepteur sera automatiquement fermé quand tous les émetteurs auront été drop()
        if !rx.is_closed() {
            while let Some(cmd) = rx.recv().await {
                match cmd {
                    Command::Ping {addr, timestamp} => {
                        cpt = cpt + 1;
                        ips.insert(addr.clone(), timestamp.clone()); }
                }
            }
        }
        drop(rx);
        return (cpt, ips);
    });

    // Démarrage des tâches :
    let start = Utc::now();
    println!("Pinger démarré à {:?}", start);
    let (nb_pinge, pings) = logger.await.unwrap();      // on lance la tâche logger
    join_all(tasks).await;      // on rassemble toutes les tâches de ping pour le scheduler et on les démarre
    let now = Utc::now();
    let elapsed = now - start;
    update_filelog(LOG_FILENAME, SEP, pings);
    println!("Pinger terminé en {} secondes. {} adresses ont répondu sur {} interrogées", elapsed.num_seconds(), nb_pinge, nb);
    Ok(())                      // on renvoie un Result Ok vide
}

/// fonction ping : consomme une instance de Client et une IpAddr
/// Renvoie un Result : un Command::Ping si Ok, une Err() sinon
 async fn ping(client: Client, addr: IpAddr) -> Result<Command, ()> {
    let payload = [0; 56];
    let mut pinger = client.pinger(addr, PingIdentifier(random())).await;
    pinger.timeout(Duration::from_secs(TIMEOUT));
    match pinger.ping(PingSequence(0), &payload).await {
        Ok((IcmpPacket::V4(packet), _duration)) => {
            let timestamp = Utc::now().timestamp().to_string();     // on récupère le timestamp au moment de la réponse
            let addr = packet.get_source().to_string();             // et l'adresse IP du paquet de réponse
            let command = Command::Ping{addr, timestamp};                 // On renvoie un Enum Command::Ping dans un Ok()
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
