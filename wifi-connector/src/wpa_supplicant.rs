use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::path::Path;

/// Implementazione usando wpa_supplicant per la connessione WiFi
/// 
/// NOTA: Questa implementazione richiede privilegi root per funzionare.
/// È più a basso livello rispetto a NetworkManager e richiede gestione manuale
/// della configurazione e del DHCP.

/// Struttura per rappresentare una rete WiFi
#[derive(Debug, Clone)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal: String,
    pub bssid: String,
    pub frequency: String,
}

/// Path del file di configurazione wpa_supplicant
const WPA_SUPPLICANT_CONF: &str = "/tmp/wpa_supplicant_demo.conf";

pub fn main() {
    println!("=== WiFi Connector con wpa_supplicant ===");
    println!("Implementazione a basso livello usando wpa_supplicant\n");
    
    // Verifica se l'utente ha i privilegi necessari
    if !check_privileges() {
        println!("⚠️  ATTENZIONE: Questa implementazione richiede privilegi root.");
        println!("   Per eseguire con privilegi: sudo cargo run --bin wifi-wpa-supplicant");
        println!("   Continuando in modalità demo (alcune operazioni falliranno).\n");
    }

    loop {
        println!("\n--- Menu wpa_supplicant ---");
        println!("1. Scansiona reti WiFi (usando iw scan)");
        println!("2. Genera configurazione wpa_supplicant");
        println!("3. Connetti usando wpa_supplicant");
        println!("4. Ottieni indirizzo IP (DHCP)");
        println!("5. Disconnetti");
        println!("6. Mostra stato wpa_supplicant");
        println!("7. Info su questa implementazione");
        println!("0. Esci");
        print!("\nScegli un'opzione: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => scan_networks(),
            "2" => generate_wpa_config(),
            "3" => connect_wpa_supplicant(),
            "4" => get_ip_address(),
            "5" => disconnect(),
            "6" => show_wpa_status(),
            "7" => show_implementation_info(),
            "0" => {
                println!("Arrivederci!");
                break;
            }
            _ => println!("Opzione non valida!"),
        }
    }
}

/// Verifica se il programma ha privilegi root
fn check_privileges() -> bool {
    let output = Command::new("id")
        .arg("-u")
        .output();
    
    match output {
        Ok(result) => {
            let uid = String::from_utf8_lossy(&result.stdout).trim().to_string();
            uid == "0"
        }
        Err(_) => false,
    }
}

/// Trova l'interfaccia WiFi disponibile
fn find_wifi_interface() -> Option<String> {
    let output = Command::new("iw")
        .arg("dev")
        .output();
    
    match output {
        Ok(result) => {
            let text = String::from_utf8_lossy(&result.stdout);
            // Cerca la prima interfaccia
            for line in text.lines() {
                if line.trim().starts_with("Interface ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        return Some(parts[1].to_string());
                    }
                }
            }
            None
        }
        Err(_) => {
            println!("Errore: comando 'iw' non trovato. Installa iw: sudo apt install iw");
            None
        }
    }
}

/// Scansiona le reti WiFi usando iw
fn scan_networks() {
    println!("\n--- Scansione Reti WiFi (iw scan) ---");
    println!("Approccio: Comando iw (wireless tools)\n");
    
    let interface = match find_wifi_interface() {
        Some(iface) => iface,
        None => {
            println!("❌ Nessuna interfaccia WiFi trovata!");
            println!("   Usa 'iw dev' per verificare le interfacce disponibili.");
            return;
        }
    };
    
    println!("Usando interfaccia: {}", interface);
    println!("Esecuzione scan (richiede privilegi)...\n");
    
    let output = Command::new("iw")
        .args(&[&interface, "scan"])
        .output();
    
    match output {
        Ok(result) => {
            if !result.status.success() {
                println!("❌ Errore durante lo scan:");
                println!("{}", String::from_utf8_lossy(&result.stderr));
                println!("\n💡 Suggerimento: Prova con 'sudo' per avere i privilegi necessari.");
                return;
            }
            
            let scan_output = String::from_utf8_lossy(&result.stdout);
            let networks = parse_scan_output(&scan_output);
            
            if networks.is_empty() {
                println!("Nessuna rete trovata.");
            } else {
                println!("Reti disponibili:");
                println!("{:-<80}", "");
                println!("{:<32} {:<18} {:<15} {:<10}", "SSID", "BSSID", "Frequenza", "Segnale");
                println!("{:-<80}", "");
                for network in networks {
                    println!("{:<32} {:<18} {:<15} {:<10}", 
                        network.ssid, 
                        network.bssid,
                        network.frequency,
                        network.signal
                    );
                }
                println!("{:-<80}", "");
            }
        }
        Err(e) => {
            println!("❌ Errore: {}", e);
            println!("   Assicurati che 'iw' sia installato: sudo apt install iw");
        }
    }
}

/// Parse dell'output di iw scan
fn parse_scan_output(output: &str) -> Vec<WifiNetwork> {
    let mut networks = Vec::new();
    let mut current_network: Option<WifiNetwork> = None;
    
    for line in output.lines() {
        let trimmed = line.trim();
        
        // Nuova rete (inizia con BSS)
        if trimmed.starts_with("BSS ") {
            if let Some(net) = current_network.take() {
                if !net.ssid.is_empty() {
                    networks.push(net);
                }
            }
            
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let bssid = parts[1].trim_end_matches("(on").to_string();
                current_network = Some(WifiNetwork {
                    ssid: String::new(),
                    signal: String::new(),
                    bssid,
                    frequency: String::new(),
                });
            }
        }
        
        // SSID
        if trimmed.starts_with("SSID: ") {
            if let Some(ref mut net) = current_network {
                net.ssid = trimmed.strip_prefix("SSID: ").unwrap_or("").to_string();
            }
        }
        
        // Segnale
        if trimmed.starts_with("signal: ") {
            if let Some(ref mut net) = current_network {
                net.signal = trimmed.strip_prefix("signal: ").unwrap_or("").to_string();
            }
        }
        
        // Frequenza
        if trimmed.starts_with("freq: ") {
            if let Some(ref mut net) = current_network {
                let freq = trimmed.strip_prefix("freq: ").unwrap_or("");
                net.frequency = format!("{} MHz", freq);
            }
        }
    }
    
    // Aggiungi l'ultima rete
    if let Some(net) = current_network {
        if !net.ssid.is_empty() {
            networks.push(net);
        }
    }
    
    networks
}

/// Genera file di configurazione wpa_supplicant
fn generate_wpa_config() {
    println!("\n--- Genera Configurazione wpa_supplicant ---");
    
    print!("Inserisci SSID della rete: ");
    io::stdout().flush().unwrap();
    let mut ssid = String::new();
    io::stdin().read_line(&mut ssid).unwrap();
    let ssid = ssid.trim();
    
    if ssid.is_empty() {
        println!("❌ SSID non valido!");
        return;
    }
    
    print!("Inserisci password (lascia vuoto per rete aperta): ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();
    
    // Genera la configurazione
    let config = if password.is_empty() {
        // Rete aperta
        format!(
            "ctrl_interface=/var/run/wpa_supplicant\n\
             network={{\n\
                 ssid=\"{}\"\n\
                 key_mgmt=NONE\n\
             }}\n",
            ssid
        )
    } else {
        // Rete con password (WPA/WPA2)
        format!(
            "ctrl_interface=/var/run/wpa_supplicant\n\
             network={{\n\
                 ssid=\"{}\"\n\
                 psk=\"{}\"\n\
             }}\n",
            ssid, password
        )
    };
    
    // Salva la configurazione
    match fs::write(WPA_SUPPLICANT_CONF, &config) {
        Ok(_) => {
            println!("✅ Configurazione salvata in: {}", WPA_SUPPLICANT_CONF);
            println!("\nContenuto del file:");
            println!("{:-<60}", "");
            // Mostra la configurazione (ma oscura la password)
            let display_config = if password.is_empty() {
                config.clone()
            } else {
                config.replace(password, "********")
            };
            println!("{}", display_config);
            println!("{:-<60}", "");
            println!("\n💡 Prossimo passo: Opzione 3 per avviare wpa_supplicant");
        }
        Err(e) => {
            println!("❌ Errore nel salvataggio del file: {}", e);
        }
    }
}

/// Connetti usando wpa_supplicant
fn connect_wpa_supplicant() {
    println!("\n--- Connessione con wpa_supplicant ---");
    
    // Verifica che il file di configurazione esista
    if !Path::new(WPA_SUPPLICANT_CONF).exists() {
        println!("❌ File di configurazione non trovato!");
        println!("   Usa prima l'opzione 2 per generare la configurazione.");
        return;
    }
    
    let interface = match find_wifi_interface() {
        Some(iface) => iface,
        None => {
            println!("❌ Nessuna interfaccia WiFi trovata!");
            return;
        }
    };
    
    println!("Interfaccia: {}", interface);
    println!("Config: {}", WPA_SUPPLICANT_CONF);
    println!("\nAvvio wpa_supplicant...");
    
    // Uccidi eventuali istanze precedenti
    let _ = Command::new("pkill")
        .arg("wpa_supplicant")
        .output();
    
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    // Avvia wpa_supplicant in background
    let output = Command::new("wpa_supplicant")
        .args(&[
            "-B",                           // Background
            "-i", &interface,               // Interfaccia
            "-c", WPA_SUPPLICANT_CONF,     // File di configurazione
        ])
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                println!("✅ wpa_supplicant avviato con successo!");
                println!("\n💡 Prossimo passo:");
                println!("   1. Aspetta qualche secondo per l'associazione");
                println!("   2. Usa l'opzione 4 per ottenere un indirizzo IP via DHCP");
                println!("   3. Usa l'opzione 6 per verificare lo stato");
            } else {
                println!("❌ Errore nell'avvio di wpa_supplicant:");
                println!("{}", String::from_utf8_lossy(&result.stderr));
                println!("\n💡 Suggerimento: Esegui con sudo per avere i privilegi necessari.");
            }
        }
        Err(e) => {
            println!("❌ Errore: {}", e);
            println!("   Assicurati che wpa_supplicant sia installato:");
            println!("   sudo apt install wpasupplicant");
        }
    }
}

/// Ottieni indirizzo IP via DHCP
fn get_ip_address() {
    println!("\n--- Ottieni Indirizzo IP (DHCP) ---");
    
    let interface = match find_wifi_interface() {
        Some(iface) => iface,
        None => {
            println!("❌ Nessuna interfaccia WiFi trovata!");
            return;
        }
    };
    
    println!("Richiedendo indirizzo IP per {}...", interface);
    
    // Prova prima dhclient
    let output = Command::new("dhclient")
        .arg(&interface)
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                println!("✅ DHCP client eseguito con successo!");
                
                // Mostra l'indirizzo ottenuto
                std::thread::sleep(std::time::Duration::from_secs(2));
                show_ip_info(&interface);
            } else {
                println!("⚠️  dhclient fallito, provo con dhcpcd...");
                
                // Prova dhcpcd come alternativa
                let output2 = Command::new("dhcpcd")
                    .arg(&interface)
                    .output();
                
                match output2 {
                    Ok(result2) => {
                        if result2.status.success() {
                            println!("✅ dhcpcd eseguito con successo!");
                            std::thread::sleep(std::time::Duration::from_secs(2));
                            show_ip_info(&interface);
                        } else {
                            println!("❌ Errore con dhcpcd:");
                            println!("{}", String::from_utf8_lossy(&result2.stderr));
                        }
                    }
                    Err(_) => {
                        println!("❌ Né dhclient né dhcpcd sono disponibili.");
                        println!("   Installa uno di questi: sudo apt install isc-dhcp-client");
                    }
                }
            }
        }
        Err(_) => {
            println!("❌ dhclient non trovato. Provo dhcpcd...");
            
            let output2 = Command::new("dhcpcd")
                .arg(&interface)
                .output();
            
            match output2 {
                Ok(result2) => {
                    if result2.status.success() {
                        println!("✅ dhcpcd eseguito con successo!");
                        std::thread::sleep(std::time::Duration::from_secs(2));
                        show_ip_info(&interface);
                    } else {
                        println!("❌ Errore con dhcpcd:");
                        println!("{}", String::from_utf8_lossy(&result2.stderr));
                    }
                }
                Err(_) => {
                    println!("❌ Nessun client DHCP trovato.");
                    println!("   Installa: sudo apt install isc-dhcp-client");
                }
            }
        }
    }
}

/// Mostra informazioni IP dell'interfaccia
fn show_ip_info(interface: &str) {
    let output = Command::new("ip")
        .args(&["addr", "show", interface])
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                println!("\n📊 Informazioni interfaccia {}:", interface);
                println!("{:-<60}", "");
                println!("{}", String::from_utf8_lossy(&result.stdout));
                println!("{:-<60}", "");
            }
        }
        Err(_) => {}
    }
}

/// Disconnetti e termina wpa_supplicant
fn disconnect() {
    println!("\n--- Disconnessione ---");
    
    println!("Terminando wpa_supplicant...");
    let output = Command::new("pkill")
        .arg("wpa_supplicant")
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                println!("✅ wpa_supplicant terminato.");
                
                // Opzionalmente, rilascia l'indirizzo IP
                if let Some(interface) = find_wifi_interface() {
                    println!("Rilascio indirizzo IP...");
                    let _ = Command::new("dhclient")
                        .args(&["-r", &interface])
                        .output();
                }
                
                println!("✅ Disconnesso.");
            } else {
                println!("⚠️  wpa_supplicant potrebbe non essere in esecuzione.");
            }
        }
        Err(e) => {
            println!("❌ Errore: {}", e);
        }
    }
    
    // Rimuovi il file di configurazione temporaneo
    if Path::new(WPA_SUPPLICANT_CONF).exists() {
        let _ = fs::remove_file(WPA_SUPPLICANT_CONF);
        println!("🗑️  File di configurazione rimosso.");
    }
}

/// Mostra lo stato di wpa_supplicant
fn show_wpa_status() {
    println!("\n--- Stato wpa_supplicant ---");
    
    let interface = match find_wifi_interface() {
        Some(iface) => iface,
        None => {
            println!("❌ Nessuna interfaccia WiFi trovata!");
            return;
        }
    };
    
    // Verifica se wpa_supplicant è in esecuzione
    let ps_output = Command::new("pgrep")
        .args(&["-a", "wpa_supplicant"])
        .output();
    
    match ps_output {
        Ok(result) => {
            if result.status.success() && !result.stdout.is_empty() {
                println!("✅ wpa_supplicant è in esecuzione:");
                println!("{}", String::from_utf8_lossy(&result.stdout));
            } else {
                println!("⚠️  wpa_supplicant non è in esecuzione.");
            }
        }
        Err(_) => {
            println!("⚠️  Impossibile verificare lo stato di wpa_supplicant.");
        }
    }
    
    // Mostra lo stato dell'interfaccia
    println!("\n📊 Stato interfaccia {}:", interface);
    let iw_output = Command::new("iw")
        .args(&[&interface, "link"])
        .output();
    
    match iw_output {
        Ok(result) => {
            if result.status.success() {
                println!("{}", String::from_utf8_lossy(&result.stdout));
            } else {
                println!("Non connesso o impossibile ottenere informazioni.");
            }
        }
        Err(_) => {
            println!("Errore nell'esecuzione di 'iw link'.");
        }
    }
    
    // Mostra indirizzo IP
    show_ip_info(&interface);
}

/// Mostra informazioni sull'implementazione wpa_supplicant
fn show_implementation_info() {
    println!("\n=== Implementazione wpa_supplicant ===\n");
    
    println!("📚 PANORAMICA");
    println!("Questa implementazione usa wpa_supplicant, un'alternativa a basso livello");
    println!("per la gestione delle connessioni WiFi su Linux.");
    println!();
    
    println!("🔧 COMPONENTI UTILIZZATI");
    println!("1. iw - Per scansionare le reti WiFi");
    println!("   Comando: iw <interface> scan");
    println!();
    println!("2. wpa_supplicant - Per autenticare e associarsi alla rete");
    println!("   Comando: wpa_supplicant -B -i <interface> -c <config_file>");
    println!("   - -B: Esegui in background (daemon mode)");
    println!("   - -i: Specifica l'interfaccia di rete");
    println!("   - -c: Specifica il file di configurazione");
    println!();
    println!("3. dhclient o dhcpcd - Per ottenere un indirizzo IP via DHCP");
    println!("   Comando: dhclient <interface>");
    println!();
    
    println!("📝 WORKFLOW COMPLETO");
    println!("1. Scansione: iw <interface> scan");
    println!("2. Configurazione: Crea /etc/wpa_supplicant/wpa_supplicant.conf");
    println!("3. Autenticazione: wpa_supplicant -B -i <interface> -c <config>");
    println!("4. DHCP: dhclient <interface>");
    println!();
    
    println!("⚙️  FILE DI CONFIGURAZIONE");
    println!("Esempio per rete WPA/WPA2:");
    println!("---");
    println!("ctrl_interface=/var/run/wpa_supplicant");
    println!("network={{");
    println!("    ssid=\"MiaRete\"");
    println!("    psk=\"MiaPassword\"");
    println!("}}");
    println!("---");
    println!();
    println!("Esempio per rete aperta:");
    println!("---");
    println!("ctrl_interface=/var/run/wpa_supplicant");
    println!("network={{");
    println!("    ssid=\"ReteAperta\"");
    println!("    key_mgmt=NONE");
    println!("}}");
    println!("---");
    println!();
    
    println!("✅ VANTAGGI");
    println!("• Non dipende da NetworkManager");
    println!("• Funziona su sistemi minimali/embedded");
    println!("• Maggior controllo sul processo di connessione");
    println!("• Ideale per server e sistemi headless");
    println!();
    
    println!("⚠️  SVANTAGGI");
    println!("• Richiede privilegi root");
    println!("• Configurazione manuale più complessa");
    println!("• Gestione separata di autenticazione e DHCP");
    println!("• Meno user-friendly");
    println!();
    
    println!("🔒 SICUREZZA");
    println!("• Il file di configurazione contiene password in chiaro");
    println!("• Deve avere permessi restrittivi (chmod 600)");
    println!("• Meglio usare wpa_passphrase per generare PSK cifrato:");
    println!("  wpa_passphrase SSID password >> /etc/wpa_supplicant/wpa_supplicant.conf");
    println!();
    
    println!("📦 PACCHETTI RICHIESTI");
    println!("• wpasupplicant - Per wpa_supplicant");
    println!("• iw - Per scanning e gestione wireless");
    println!("• isc-dhcp-client o dhcpcd - Per DHCP");
    println!();
    println!("Installazione su Debian/Ubuntu:");
    println!("  sudo apt install wpasupplicant iw isc-dhcp-client");
    println!();
    
    println!("🎓 CONFRONTO CON NETWORKMANAGER");
    println!("NetworkManager è più alto livello e gestisce tutto automaticamente.");
    println!("wpa_supplicant è più a basso livello e richiede gestione manuale.");
    println!("NetworkManager usa wpa_supplicant internamente!");
    println!();
}
