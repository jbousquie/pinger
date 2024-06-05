/// Module des paramètres de la configuration
pub mod settings {
    use std::fs;
    use toml::Table;

    // variables par défaut
    const DEF_IP_FILENAME: &str = "adresses.txt"; //nom du fichier des adresses à pinger
    const DEF_LOG_FILENAME: &str = "pinger.log"; // nom du fichier de log des pings
    const DEF_SEP: &str = ",";                   // caractère séparateur dans le fichier de log
    const DEF_TIMEOUT: u64 = 1;                  // time du ping

    pub struct Settings {
        pub addr_filename: String,              // nom du fichier des adresses IP à pinguer
        pub log_filename: String,               // nom du fichier de log des résultats du ping
        pub logfile_sep: String,                // caractère de séparation ip/timestamp dans le fichier de log
        pub ping_timeout: u64,                  // timeout en seconde(s) avant de considérer un ping comme non répondu
    }

    /// Renvoie une string à partir de la valeur d'un param du fichier toml
    fn string_from_settings(setting: &toml::Value) -> String {
        setting.as_str().unwrap().to_string()
    }

    // Lit le fichier de configuration et renvoie un objet Settings
    pub fn load_settings(settings_filename: &str) -> Settings {
        let default_settings = Settings {
            addr_filename: DEF_IP_FILENAME.to_string(),
            log_filename: DEF_LOG_FILENAME.to_string(),
            logfile_sep: DEF_SEP.to_string(),
            ping_timeout: DEF_TIMEOUT
        };
        if let Ok(settings_str) = fs::read_to_string(settings_filename) {
            if let Ok(config) = &settings_str.parse::<Table>() {
                let settings = Settings {
                    addr_filename: string_from_settings(&config["addr_filename"]),
                    log_filename: string_from_settings(&config["log_filename"]),
                    logfile_sep: string_from_settings(&config["logfile_sep"]),
                    ping_timeout: config["ping_timeout"].as_integer().unwrap() as u64,
                };
                return settings;
            } else {
                println!("Erreur dans l'analyse des paramètres du fichier de configuration {settings_filename}/\nUtilisation des paramètres par défaut.");
            }
        } else {
            println!("Fichier de configuration {settings_filename} non trouvé.\nUtilisation des paramètres par défaut.");
        }
        default_settings
    }
}