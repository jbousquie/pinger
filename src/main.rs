// https://docs.rs/surge-ping/latest/surge_ping/
// https://github.com/kolapapa/surge-ping/blob/main/examples/multi_ping.rs
// https://tokio.rs/tokio/tutorial/channels

const SETTINGS_FILENAME: &str = "./pinger.conf";
mod pinged;
mod settings;

use std::process;
use std::net::IpAddr;
use chrono::Utc;
use rand::random;
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use crate::pinged::{get_ping_ips, update_data_from_logfile, update_filelog, get_ips_from_file};
use settings::settings::load_settings;

/// Définition d'un message à passer dans le channel inter-tâches
enum Command {
    Ping {
        addr: String,               // IP de l'hôte pingé
        timestamp: String,          // timestamp de la réponse
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = load_settings(SETTINGS_FILENAME);
    let ip_filename = &settings.addr_filename;
    let log_filename = &settings.log_filename;
    let sep = &settings.logfile_sep;
    let ping_timeout = settings.ping_timeout;
    let task_group_nb = settings.task_group_nb;
    let task_group_delay = settings.task_group_delay;
    // Récupération de la Hashmap des adresses IPv4 à pinger
    let vec_ips = get_ips_from_file(ip_filename);
    let mut ips = get_ping_ips(&vec_ips);
    let nb = ips.len();
    if nb == 0 {
        println!("fichier des adresses à pinger {ip_filename} vide.");
        process::exit(0x0100);
    }
    // Récupération éventuellement des logs précédents depuis le fichier et mise à jour de la HashMap
    ips = update_data_from_logfile(log_filename, sep, ips);

    // Création du channel inter-tâches de capacité 32
    // tx est le canal d'envoi, rx celui de réception
    let (tx, mut rx) = mpsc::channel(32);
    // Client IPv4 uniquement
    let client_v4 = Client::new(&Config::default())?;

    // Démarrage des tâches :
    let mut task_ctp = 1;
    let start = Utc::now();
    println!("Pinger démarré à {:?}", start);
    // Liste de ping (émetteur du channel)
    // On lance une tâche de ping par adresse IPv4 du tableau valide
    for ip in &vec_ips {
        // analyse de la string IP clé de la HashMap
        match ip.parse::<core::net::IpAddr>() {
            // si l'adresse est une IPv4 valide
            Ok(IpAddr::V4(addr)) => {
                // on stocke une tâche dans le tableau des tâches à partir d'un clone du client
                let txc = tx.clone(); // on clone le canal émetteur du channel
                let client_clone = client_v4.clone();
                let _task = tokio::spawn( async move {
                    // la tâche attend un résultat du ping et si OK le transmet dans le channel à la tâche logger
                    let ping_res = ping(client_clone, IpAddr::V4(addr), ping_timeout).await; // appel effectif du ping
                    if let Ok(cmd) = ping_res {
                        txc.send(cmd).await.unwrap();   // envoi du message Command::Ping à la tâche logger
                        drop(txc);                      // puis on ferme explicitement l'émetteur
                    }
                });
                task_ctp += 1;
                if task_ctp % task_group_nb == 0 {
                    let _ = sleep(Duration::from_millis(task_group_delay)).await;
                }
            },
            // si l'adresse est une IPv6 valide
            Ok(IpAddr::V6(addr)) => {
                println!("ping IPv6 non activé {}", addr); // message d'information
            },
            // Si l'adresse n'est pas valide et n'est pas un commentaire -> message d'information
            Err(e) => {
                if !(ip.contains("//") || ip.contains("#")) {
                    println!("{} erreur d\'analyse de l\'adresse IP : {}", ip, e);
                }
            }
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


    let (nb_pinge, pings) = logger.await.unwrap();      // on lance la tâche logger
    let now = Utc::now();
    let elapsed = now - start;
    update_filelog(log_filename, sep, &vec_ips, pings);
    println!("Pinger terminé en {} millisecondes : {} adresses ont répondu sur {} interrogées", elapsed.num_milliseconds(), nb_pinge, nb);
    Ok(())                      // on renvoie un Result Ok vide
}

/// fonction ping : consomme une instance de Client et une IpAddr
/// Renvoie un Result : un Command::Ping si Ok, une Err() sinon
 async fn ping(client: Client, addr: IpAddr, timeout: u64) -> Result<Command, ()> {
    let payload = [0; 56];                                        // 56 octets de payload pour faire 64 en tout avec l'entête ICMP
    let mut pinger = client.pinger(addr, PingIdentifier(random())).await;
    pinger.timeout(std::time::Duration::from_millis(timeout));
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
