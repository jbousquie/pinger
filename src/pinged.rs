use std::collections::HashMap;
use std::fs;

/// Renvoie une Hahsmap<&str, &str> d'IP à pinger à partir d'un Vec<String> d'IP
pub fn get_ping_ips(vec_ips: &Vec<String>) -> HashMap<String, String> {
    let mut ping_ips = HashMap::new();
    for i in vec_ips.iter() {
        if i.len() > 0 {
            ping_ips.insert(i.clone(), "0".to_string());
        }
    }
    ping_ips
}

/// Lit le fichier d'adresses à pinger et renvoie un Vec<String>
pub fn get_ips_from_file(ipfilename: &str) -> Vec<String> {
    /*
    // Génération de données dans le fichier IP
    let mut data = "".to_string();
    let prefixes = [
        "192.168.0.",
        "192.168.100.",
        "10.5.0.",
        "193.54.203.",
        "193.49.48.",
        "10.46.0.",
        "10.2.3.",
        "10.2.4.",
        "10.2.5.",
        "10.2.6.",
        "10.2.7.",
        "10.2.8.",
        "10.2.9.",
        "10.2.10.",
        "10.2.11.",
        "10.2.12.",
        "10.2.13.",
        "10.2.14.",
        "10.2.15.",
        "8.8.8.",
    ];
    for j in 0..prefixes.len() {
        let prefix = prefixes[j];
        for i in 1..256 {
            let ip = prefix.to_string() + i.to_string().as_str() + "\n";
            data = data + ip.as_str();
        }
    }
    fs::write(ipfilename, data.as_str()).unwrap();
    */



    let mut ips = Vec::new();
    if let Ok(str_lines) = fs::read_to_string(ipfilename) {
        let lines: Vec<&str> = str_lines.split("\n").collect();
        for line in lines {
            if line.len() > 0 {
                ips.push(line.to_string());
            }
        }
    }
    else {
        println!("Erreur de lecture du fichier des adresses IP à pinger : {ipfilename}");
    }
    ips
}

/// Met à jour le fichier de log de ping à partir des adresses à pinguer ordonnées et des résultats du ping
pub fn update_filelog(logfilename: &str, sep: &str,addrs: &Vec<String>, pings: HashMap<String, String>) {
    let mut data = "".to_string();
    // on parcourt la liste des adresses à pinger dans l'ordre initial du fichier
    for ip in addrs.iter() {
        if pings.contains_key(ip) {
            let ts = pings.get(ip).unwrap();
            data = data + ip + sep + ts.as_str() + "\n";
        }
    }
    // écriture des résultats de ping dans le fichier de log
    if let Ok(_file) = fs::write(logfilename, data.as_str()) {
        println!("Fichier des logs {} mis à jour", logfilename);
    }
    else {
        println!("Erreur : impossible d'écrire dans le fichier de logs : {}", logfilename);
    }

}
/// Récupère d'abord les données depuis le fichier de log
/// et met à jour la HashMap des pings avec les valeurs déjà enregistrées
pub fn update_data_from_logfile (logfilename: &str, sep: &str, mut pings: HashMap<String, String>) -> HashMap<String, String> {
    if let Ok(log_lines) = fs::read_to_string(logfilename) {
        let lines: Vec<&str> = log_lines.split("\n").collect();
        for line in lines {
            let data: Vec<&str> = line.split(sep).collect();
            if data.len() == 2 {
                let ip = data[0].to_string();
                let ts = data[1].to_string();
                pings.insert(ip, ts);
            }
        }
    } else {
        println!("Log des pings non récupéré depuis le fichier : {logfilename}");
    }
    pings
}