use std::collections::HashMap;
use std::fs;

// Renvoie une Hahsmap<&str, &str> d'IP à pinger
pub fn get_ping_ips(filename: &str) -> HashMap<String, String> {
    let ips = get_ips_from_file(filename);
    let mut ping_ips = HashMap::new();
    for i in ips.iter() {
        ping_ips.insert(i.clone(), "0".to_string());
    }
    ping_ips
}

// Lit le fichier d'adresses à pinger et renvoie un Vec<String>
fn get_ips_from_file(filename: &str) -> Vec<String> {
    /*
    let mut data = "".to_string();
    let prefixes = ["192.168.0.", "192.168.100.","10.5.0."];
    for j in 0..2 {
        let prefix = prefixes[j];
        for i in 1..256 {
            let ip = prefix.to_string() + i.to_string().as_str() + "\n";
            data = data + ip.as_str();
        }
    }
    fs::write(filename, data.as_str()).unwrap();

     */



    let mut ips = Vec::new();
    if let Ok(str_lines) = fs::read_to_string(filename) {
        let lines: Vec<&str> = str_lines.split("\n").collect();
        for line in lines {
            ips.push(line.to_string());
        }
    }
    else {
        println!("Erreur de lecture du fichier : {filename}");
    }
    ips
}

