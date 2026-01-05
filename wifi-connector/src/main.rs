use std::io::{self, Write};
use std::process::Command;

/// Rappresenta un approccio per connettersi al WiFi
#[derive(Debug)]
enum WifiApproach {
    NetworkManager,  // Usa nmcli (NetworkManager)
    WpaSupplicant,   // Usa wpa_supplicant direttamente (più basso livello)
    IwdCtl,          // Usa iwctl (iwd daemon)
}

/// Struttura per rappresentare una rete WiFi
#[derive(Debug, Clone)]
struct WifiNetwork {
    ssid: String,
    signal: String,
    security: String,
}

fn main() {
    println!("=== WiFi Connector per Linux ===");
    println!("App educativa per esplorare diversi approcci di connessione WiFi\n");

    loop {
        println!("\n--- Menu Principale ---");
        println!("1. Scansiona reti WiFi disponibili");
        println!("2. Connetti a una rete WiFi");
        println!("3. Mostra reti salvate");
        println!("4. Disconnetti dalla rete corrente");
        println!("5. Mostra stato connessione");
        println!("6. Info sugli approcci implementativi");
        println!("0. Esci");
        print!("\nScegli un'opzione: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => scan_networks(),
            "2" => connect_to_network(),
            "3" => show_saved_networks(),
            "4" => disconnect_network(),
            "5" => show_connection_status(),
            "6" => show_implementation_info(),
            "0" => {
                println!("Arrivederci!");
                break;
            }
            _ => println!("Opzione non valida!"),
        }
    }
}

/// Scansiona le reti WiFi disponibili usando nmcli (Approccio NetworkManager)
fn scan_networks() {
    println!("\n--- Scansione Reti WiFi ---");
    println!("Approccio: NetworkManager (nmcli)\n");

    // Richiede rescan delle reti
    let rescan = Command::new("nmcli")
        .args(&["device", "wifi", "rescan"])
        .output();

    match rescan {
        Ok(_) => {
            std::thread::sleep(std::time::Duration::from_secs(2));
            
            // Lista le reti disponibili
            let output = Command::new("nmcli")
                .args(&["--terse", "--fields", "SSID,SIGNAL,SECURITY", "device", "wifi", "list"])
                .output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        let networks_str = String::from_utf8_lossy(&result.stdout);
                        let mut networks: Vec<WifiNetwork> = Vec::new();

                        for line in networks_str.lines() {
                            let parts: Vec<&str> = line.split(':').collect();
                            if parts.len() >= 3 && !parts[0].is_empty() {
                                networks.push(WifiNetwork {
                                    ssid: parts[0].to_string(),
                                    signal: parts[1].to_string(),
                                    security: parts[2].to_string(),
                                });
                            }
                        }

                        if networks.is_empty() {
                            println!("Nessuna rete trovata.");
                        } else {
                            println!("Reti disponibili:");
                            println!("{:-<60}", "");
                            println!("{:<30} {:<10} {:<20}", "SSID", "Segnale", "Sicurezza");
                            println!("{:-<60}", "");
                            for network in networks {
                                // Note: nmcli restituisce il segnale come numero (0-100)
                                // In un'implementazione robusta, si dovrebbe validare che sia numerico
                                println!("{:<30} {:<10}% {:<20}", 
                                    network.ssid, 
                                    network.signal,
                                    if network.security.is_empty() { "Aperta" } else { &network.security }
                                );
                            }
                            println!("{:-<60}", "");
                        }
                    } else {
                        println!("Errore durante la scansione: {}", String::from_utf8_lossy(&result.stderr));
                    }
                }
                Err(e) => {
                    println!("Errore: nmcli non disponibile - {}", e);
                    println!("Assicurati che NetworkManager sia installato.");
                }
            }
        }
        Err(e) => {
            println!("Errore durante il rescan: {}", e);
        }
    }
}

/// Connette a una rete WiFi
fn connect_to_network() {
    println!("\n--- Connessione a Rete WiFi ---");
    
    print!("Inserisci SSID della rete: ");
    io::stdout().flush().unwrap();
    let mut ssid = String::new();
    io::stdin().read_line(&mut ssid).unwrap();
    let ssid = ssid.trim();

    if ssid.is_empty() {
        println!("SSID non valido!");
        return;
    }

    print!("Inserisci password (lascia vuoto per reti aperte): ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    println!("\nTentativo di connessione a '{}'...", ssid);
    println!("Approccio: NetworkManager (nmcli)\n");

    // NOTA DI SICUREZZA: Passare la password come argomento CLI è insicuro!
    // Alternative più sicure per applicazioni reali:
    // 1. Usare D-Bus API di NetworkManager per passare credenziali in modo sicuro
    // 2. Leggere da file di configurazione con permessi restrittivi
    // 3. Usare il keyring del sistema (es. libsecret)
    // Questa è solo una demo educativa!
    let output = if password.is_empty() {
        Command::new("nmcli")
            .args(&["device", "wifi", "connect", ssid])
            .output()
    } else {
        Command::new("nmcli")
            .args(&["device", "wifi", "connect", ssid, "password", password])
            .output()
    };

    match output {
        Ok(result) => {
            if result.status.success() {
                println!("✓ Connesso con successo a '{}'!", ssid);
                println!("{}", String::from_utf8_lossy(&result.stdout));
            } else {
                println!("✗ Errore durante la connessione:");
                println!("{}", String::from_utf8_lossy(&result.stderr));
            }
        }
        Err(e) => {
            println!("Errore: {}", e);
            println!("Assicurati di avere i permessi necessari e NetworkManager installato.");
        }
    }
}

/// Mostra le reti salvate
fn show_saved_networks() {
    println!("\n--- Reti Salvate ---");
    println!("Approccio: NetworkManager (nmcli)\n");

    let output = Command::new("nmcli")
        .args(&["--terse", "--fields", "NAME,TYPE", "connection", "show"])
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                let connections = String::from_utf8_lossy(&result.stdout);
                let mut wifi_connections: Vec<String> = Vec::new();

                for line in connections.lines() {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() >= 2 && parts[1].contains("wifi") {
                        wifi_connections.push(parts[0].to_string());
                    }
                }

                if wifi_connections.is_empty() {
                    println!("Nessuna rete WiFi salvata.");
                } else {
                    println!("Connessioni WiFi salvate:");
                    for (i, conn) in wifi_connections.iter().enumerate() {
                        println!("  {}. {}", i + 1, conn);
                    }
                }
            } else {
                println!("Errore: {}", String::from_utf8_lossy(&result.stderr));
            }
        }
        Err(e) => {
            println!("Errore: {}", e);
        }
    }
}

/// Disconnette dalla rete corrente
fn disconnect_network() {
    println!("\n--- Disconnessione ---");
    println!("Approccio: NetworkManager (nmcli)\n");

    // Prima otteniamo il nome del dispositivo WiFi
    let device_output = Command::new("nmcli")
        .args(&["--terse", "--fields", "DEVICE,TYPE", "device", "status"])
        .output();

    match device_output {
        Ok(result) => {
            if result.status.success() {
                let devices = String::from_utf8_lossy(&result.stdout);
                let mut wifi_device = String::new();

                for line in devices.lines() {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() >= 2 && parts[1] == "wifi" {
                        wifi_device = parts[0].to_string();
                        break;
                    }
                }

                if wifi_device.is_empty() {
                    println!("Nessun dispositivo WiFi trovato.");
                    return;
                }

                let disconnect = Command::new("nmcli")
                    .args(&["device", "disconnect", &wifi_device])
                    .output();

                match disconnect {
                    Ok(res) => {
                        if res.status.success() {
                            println!("✓ Disconnesso con successo!");
                        } else {
                            println!("✗ Errore: {}", String::from_utf8_lossy(&res.stderr));
                        }
                    }
                    Err(e) => {
                        println!("Errore: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Errore: {}", e);
        }
    }
}

/// Mostra lo stato della connessione corrente
fn show_connection_status() {
    println!("\n--- Stato Connessione ---");
    println!("Approccio: NetworkManager (nmcli)\n");

    let output = Command::new("nmcli")
        .args(&["--terse", "--fields", "DEVICE,TYPE,STATE,CONNECTION", "device", "status"])
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                let devices = String::from_utf8_lossy(&result.stdout);
                
                println!("{:-<70}", "");
                println!("{:<15} {:<15} {:<20} {:<20}", "Dispositivo", "Tipo", "Stato", "Connessione");
                println!("{:-<70}", "");

                for line in devices.lines() {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() >= 4 {
                        println!("{:<15} {:<15} {:<20} {:<20}", 
                            parts[0], parts[1], parts[2], parts[3]
                        );
                    }
                }
                println!("{:-<70}", "");
            } else {
                println!("Errore: {}", String::from_utf8_lossy(&result.stderr));
            }
        }
        Err(e) => {
            println!("Errore: {}", e);
        }
    }
}

/// Mostra informazioni sui diversi approcci implementativi
fn show_implementation_info() {
    println!("\n=== Approcci per la Connessione WiFi su Linux ===\n");

    println!("Questa applicazione dimostra diversi approcci per gestire le connessioni WiFi:");
    println!();

    println!("1. NETWORKMANAGER (nmcli) - [IMPLEMENTATO]");
    println!("   Pro:");
    println!("   • Alto livello, facile da usare");
    println!("   • Gestisce automaticamente la configurazione");
    println!("   • Supporto per diverse tipologie di rete");
    println!("   • Integrazione con il sistema");
    println!("   Contro:");
    println!("   • Richiede NetworkManager installato");
    println!("   • Meno controllo sui dettagli a basso livello");
    println!("   Implementazione:");
    println!("   • Usa il comando 'nmcli' tramite std::process::Command");
    println!("   • Parsing dell'output in formato terse (--terse)");
    println!();

    println!("2. WPA_SUPPLICANT - [NON IMPLEMENTATO - Esempio Teorico]");
    println!("   Pro:");
    println!("   • Controllo più granulare");
    println!("   • Non dipende da NetworkManager");
    println!("   • Può essere usato su sistemi minimali");
    println!("   Contro:");
    println!("   • Richiede configurazione manuale più complessa");
    println!("   • Necessita privilegi root");
    println!("   • Richiede gestione manuale di wpa_supplicant.conf");
    println!("   Come si implementerebbe:");
    println!("   • Scrivere file di configurazione /etc/wpa_supplicant/wpa_supplicant.conf");
    println!("   • Eseguire: wpa_supplicant -B -i <interface> -c <config>");
    println!("   • Gestire DHCP con dhclient o dhcpcd");
    println!();

    println!("3. IWD (iNet Wireless Daemon) - [NON IMPLEMENTATO - Esempio Teorico]");
    println!("   Pro:");
    println!("   • Moderno e leggero");
    println!("   • Performance migliori");
    println!("   • Configurazione più semplice di wpa_supplicant");
    println!("   Contro:");
    println!("   • Meno diffuso di NetworkManager");
    println!("   • Meno documentazione");
    println!("   Come si implementerebbe:");
    println!("   • Usare 'iwctl' come comando CLI");
    println!("   • Esempio: iwctl station <device> connect <SSID>");
    println!();

    println!("4. LIBRERIE RUST - [NON IMPLEMENTATO - Esempio Teorico]");
    println!("   Pro:");
    println!("   • Controllo completo e type-safe");
    println!("   • Nessuna dipendenza da comandi esterni");
    println!("   • Migliore error handling");
    println!("   Contro:");
    println!("   • Implementazione più complessa");
    println!("   • Richiede bindings a basso livello");
    println!("   Crates potenzialmente utilizzabili:");
    println!("   • network-manager crate (bindings per NetworkManager D-Bus API)");
    println!("   • libc per accesso diretto a socket e ioctl");
    println!("   • neli per comunicazione Netlink");
    println!();

    println!("APPROCCIO ATTUALE:");
    println!("Questa implementazione usa NetworkManager (nmcli) per semplicità e portabilità.");
    println!("È l'approccio più comune nelle distribuzioni Linux moderne con desktop environment.");
    println!();

    println!("CONSIDERAZIONI DI SICUREZZA:");
    println!("• Le password vengono passate come argomenti al comando (visibili in ps)");
    println!("• In produzione, usare metodi più sicuri (file di configurazione, D-Bus API)");
    println!("• Questa è una demo educativa, non per uso in produzione");
    println!();
}
