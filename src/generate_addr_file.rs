use std::{fs, process};

const INPUT_FILE: &str = "a_generer.txt";
const OUTPUT_FILE: &str = "adresses.txt";

struct IpList {
    bytes : Vec<Vec<String>>
}
fn parse_input_file(filename: &str) -> String {
    let mut generated = "".to_string();
    if let Ok(input_file) = fs::read_to_string(filename) {
        let lines: Vec<&str> = input_file.split("\n").collect();
        for line in lines {
            // si la ligne contient un caractère spécial "-" ou "*", on l'analyse
            if line.contains("-") || line.contains("*") {
                generated = generated + "\n" + generate_addrs(line).as_str();
            }
            // sinon on la recopie directement dans la string générée
            else {
                generated = generated + line + "\n";
            }

        }
        generated
    }
    else {
        println!("Erreur de lecture du fichier d'entrée {filename}");
        process::exit(0x0100);
    }
}

fn generate_addrs(line: &str) -> String {
    let mut ip_list = IpList {
        bytes: Vec::new(),
    };
    let blank = "".to_string();
    let octets: Vec<&str> = line.split(".").collect();
    for i in 0..4 {
        let octet = octets[i];                      // octet courant de la ligne analysée
        let mut byte = Vec::new();            // tableau de stockage de l'octet courant
        if let Ok(_bi) = octet.parse::<i32>() {          // si l'octet courant est un entier, on stocke sa valeur
            let cur_oct = octet.to_string();
            byte.push(cur_oct);
        }
        else if let Ok((start, end)) = get_limits(octet) {      // sinon on tente de générer la liste à des chars "*" ou "-"
            for o in start..end + 1 {
                let cur_oct = o.to_string();
                byte.push(cur_oct);
            }
        }
        else {                                                                     // si analyse impossible on ressort de la fonction
            print_line_error(&line);
            return blank;
        }
        ip_list.bytes.push(byte);
    }
    let ips = populate_line(&ip_list);
    println!("{ips}");
    ips
}

// renvoie les index start-end d'un octet contenant "*" ou "-" dans un Result
fn get_limits(mut octet: &str) -> Result<(i32, i32), ()>{
    if octet == "*" {
        octet = "0-254";
    }
    // soit l'octet courant contient "-"
    if octet.contains("-") {
        let mut start = 0;
        let mut end = 254;
        let start_end: Vec<&str> = octet.split("-").collect();
        if start_end.len() == 2 {
            let str_start = start_end[0];
            let str_end = start_end[1];
            if let Ok(st) = str_start.parse() {
                start = st;
            } else {
                return Err(());
            }
            if let Ok(ed) = str_end.parse() {
                end = ed;
            } else {
                return Err(());
            }
        }
        return Ok((start, end))
    }
    Err(())
}

// Cette fonction concatène dans une string
// les combinaisons possibles d'IP à partir du tableau de stockage
fn populate_line(ip_list: &IpList) -> String {
    let mut ips = "".to_string();
    let bytes = &ip_list.bytes;
    let b0 = &bytes[0];
    let b1 = &bytes[1];
    let b2 = &bytes[2];
    let b3 = &bytes[3];
    for i0 in 0..b0.len() {
        for i1 in 0..b1.len() {
            for i2 in 0..b2.len() {
                for i3 in 0..b3.len() {
                    let ip = b0[i0].clone() + "." + &b1[i1] + "." + &b2[i2] + "." +&b3[i3] + "\n";
                    ips = ips + &ip;
                }
            }
        }
    }
    ips
}
fn print_line_error(line: &str) {
    println!("Erreur sur adresse à générer à partir de la ligne {line}. Ligne ignorée.");
}

fn main() {
    let generated_addrs = parse_input_file(INPUT_FILE);
}
