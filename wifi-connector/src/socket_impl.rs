use std::io::{self, Write, BufRead, BufReader};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::Duration;

/// Implementazione usando Unix Socket per comunicare con wpa_supplicant
/// 
/// NOTA: Questa implementazione dimostra come comunicare direttamente con
/// wpa_supplicant tramite il suo control socket Unix.
/// 
/// Il control socket usa comandi testuali interattivi ed è più semplice
/// di D-Bus ma meno robusto per applicazioni complesse.

const WPA_CTRL_SOCKET: &str = "/var/run/wpa_supplicant";

/// Struttura per rappresentare una rete WiFi
#[derive(Debug, Clone)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal: String,
    pub bssid: String,
    pub frequency: String,
    pub flags: String,
}

/// Client per comunicare con wpa_supplicant via Unix socket
pub struct WpaCtrlSocket {
    stream: Option<UnixStream>,
    interface: String,
}

impl WpaCtrlSocket {
    pub fn new(interface: &str) -> Result<Self, String> {
        Ok(WpaCtrlSocket {
            stream: None,
            interface: interface.to_string(),
        })
    }
    
    pub fn connect(&mut self) -> Result<(), String> {
        let socket_path = format!("{}/{}", WPA_CTRL_SOCKET, self.interface);
        
        if !Path::new(&socket_path).exists() {
            return Err(format!(
                "Socket di controllo non trovato: {}\nAssicurati che wpa_supplicant sia in esecuzione con ctrl_interface configurato.",
                socket_path
            ));
        }
        
        match UnixStream::connect(&socket_path) {
            Ok(stream) => {
                stream.set_read_timeout(Some(Duration::from_secs(5))).ok();
                stream.set_write_timeout(Some(Duration::from_secs(5))).ok();
                self.stream = Some(stream);
                Ok(())
            }
            Err(e) => Err(format!("Impossibile connettersi al socket: {}", e))
        }
    }
    
    pub fn send_command(&mut self, command: &str) -> Result<String, String> {
        if self.stream.is_none() {
            return Err("Non connesso al socket. Chiama connect() prima.".to_string());
        }
        
        let stream = self.stream.as_mut().unwrap();
        
        // Invia il comando
        let cmd_with_newline = format!("{}\n", command);
        stream.write_all(cmd_with_newline.as_bytes())
            .map_err(|e| format!("Errore nell'invio del comando: {}", e))?;
        
        // Leggi la risposta
        let mut reader = BufReader::new(stream);
        let mut response = String::new();
        
        reader.read_line(&mut response)
            .map_err(|e| format!("Errore nella lettura della risposta: {}", e))?;
        
        Ok(response.trim().to_string())
    }
    
    pub fn disconnect(&mut self) {
        self.stream = None;
    }
}

pub fn main() {
    println!("=== WiFi Connector con Unix Socket ===");
    println!("Implementazione usando Unix Socket per comunicare con wpa_supplicant\n");
    
    println!("⚠️  NOTA:");
    println!("Questa implementazione comunica direttamente con il control socket");
    println!("di wpa_supplicant. Richiede che wpa_supplicant sia già in esecuzione");
    println!("con ctrl_interface configurato (es: /var/run/wpa_supplicant).\n");

    loop {
        println!("\n--- Menu Unix Socket ---");
        println!("1. Connetti al control socket");
        println!("2. Esegui comando PING");
        println!("3. Scansiona reti WiFi (SCAN)");
        println!("4. Mostra risultati scan (SCAN_RESULTS)");
        println!("5. Mostra stato (STATUS)");
        println!("6. Lista reti configurate (LIST_NETWORKS)");
        println!("7. Aggiungi rete");
        println!("8. Seleziona rete");
        println!("9. Disconnetti socket");
        println!("10. Info sull'implementazione Unix Socket");
        println!("0. Esci");
        print!("\nScegli un'opzione: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => connect_socket(),
            "2" => ping_command(),
            "3" => scan_networks(),
            "4" => show_scan_results(),
            "5" => show_status(),
            "6" => list_networks(),
            "7" => add_network(),
            "8" => select_network(),
            "9" => disconnect_socket(),
            "10" => show_implementation_info(),
            "0" => {
                println!("Arrivederci!");
                break;
            }
            _ => println!("Opzione non valida!"),
        }
    }
}

fn get_interface() -> String {
    print!("Inserisci il nome dell'interfaccia WiFi (es: wlan0): ");
    io::stdout().flush().unwrap();
    let mut interface = String::new();
    io::stdin().read_line(&mut interface).unwrap();
    interface.trim().to_string()
}

fn connect_socket() {
    println!("\n--- Connessione al Control Socket ---");
    
    let interface = get_interface();
    
    println!("\n📋 CODICE:");
    println!("```rust");
    println!("use std::os::unix::net::UnixStream;");
    println!();
    println!("let socket_path = \"{}/{}\";", WPA_CTRL_SOCKET, interface);
    println!("let stream = UnixStream::connect(socket_path)?;");
    println!();
    println!("println!(\"✅ Connesso al control socket!\");");
    println!("```");
    println!();
    
    match WpaCtrlSocket::new(&interface) {
        Ok(mut client) => {
            match client.connect() {
                Ok(_) => {
                    println!("✅ Connesso con successo al socket!");
                    println!("   Socket: {}/{}", WPA_CTRL_SOCKET, interface);
                }
                Err(e) => println!("❌ {}", e)
            }
        }
        Err(e) => println!("❌ {}", e)
    }
}

fn ping_command() {
    println!("\n--- Test PING ---");
    
    let interface = get_interface();
    
    println!("\n📋 CODICE:");
    println!("```rust");
    println!("// Invia comando PING");
    println!("stream.write_all(b\"PING\\n\")?;");
    println!();
    println!("// Leggi risposta");
    println!("let mut buf = String::new();");
    println!("reader.read_line(&mut buf)?;");
    println!("// Risposta attesa: \"PONG\"");
    println!("```");
    println!();
    
    match WpaCtrlSocket::new(&interface) {
        Ok(mut client) => {
            if let Ok(_) = client.connect() {
                match client.send_command("PING") {
                    Ok(response) => {
                        println!("✅ Risposta: {}", response);
                        if response == "PONG" {
                            println!("   Socket funzionante correttamente!");
                        }
                    }
                    Err(e) => println!("❌ {}", e)
                }
            } else {
                println!("❌ Impossibile connettersi al socket");
            }
        }
        Err(e) => println!("❌ {}", e)
    }
}

fn scan_networks() {
    println!("\n--- Scansiona Reti WiFi ---");
    
    let interface = get_interface();
    
    println!("\n📋 CODICE:");
    println!("```rust");
    println!("// Avvia la scansione");
    println!("stream.write_all(b\"SCAN\\n\")?;");
    println!();
    println!("// Leggi risposta");
    println!("let mut buf = String::new();");
    println!("reader.read_line(&mut buf)?;");
    println!("// Risposta: \"OK\" se la scansione è iniziata");
    println!("```");
    println!();
    
    match WpaCtrlSocket::new(&interface) {
        Ok(mut client) => {
            if let Ok(_) = client.connect() {
                match client.send_command("SCAN") {
                    Ok(response) => {
                        println!("✅ Risposta: {}", response);
                        if response == "OK" {
                            println!("   Scansione avviata! Attendi qualche secondo e usa opzione 4");
                            println!("   per vedere i risultati.");
                        }
                    }
                    Err(e) => println!("❌ {}", e)
                }
            } else {
                println!("❌ Impossibile connettersi al socket");
            }
        }
        Err(e) => println!("❌ {}", e)
    }
}

fn show_scan_results() {
    println!("\n--- Risultati Scansione ---");
    
    let _interface = get_interface();
    
    println!("\n📋 CODICE:");
    println!("```rust");
    println!("// Richiedi i risultati della scansione");
    println!("stream.write_all(b\"SCAN_RESULTS\\n\")?;");
    println!();
    println!("// Leggi tutte le righe della risposta");
    println!("let mut results = String::new();");
    println!("loop {{");
    println!("    let mut line = String::new();");
    println!("    reader.read_line(&mut line)?;");
    println!("    if line.is_empty() || line.starts_with(\"bssid\") {{");
    println!("        break;");
    println!("    }}");
    println!("    results.push_str(&line);");
    println!("}}");
    println!("```");
    println!();
    
    println!("⚠️  NOTA: Questa funzione richiede una implementazione più complessa");
    println!("per leggere correttamente tutte le righe della risposta multi-linea.");
    println!();
    println!("Il formato della risposta è:");
    println!("bssid / frequency / signal level / flags / ssid");
    println!("00:11:22:33:44:55  2437  -45  [WPA2-PSK-CCMP][ESS]  MyNetwork");
    println!();
}

fn show_status() {
    println!("\n--- Stato Connessione ---");
    
    let _interface = get_interface();
    
    println!("\n📋 CODICE:");
    println!("```rust");
    println!("// Richiedi lo stato corrente");
    println!("stream.write_all(b\"STATUS\\n\")?;");
    println!();
    println!("// Leggi la risposta multi-linea");
    println!("let mut status = String::new();");
    println!("loop {{");
    println!("    let mut line = String::new();");
    println!("    reader.read_line(&mut line)?;");
    println!("    if line.is_empty() {{");
    println!("        break;");
    println!("    }}");
    println!("    status.push_str(&line);");
    println!("}}");
    println!("```");
    println!();
    
    println!("Esempio di output STATUS:");
    println!("bssid=00:11:22:33:44:55");
    println!("ssid=MyNetwork");
    println!("id=0");
    println!("mode=station");
    println!("wpa_state=COMPLETED");
    println!("address=aa:bb:cc:dd:ee:ff");
    println!("uuid=...");
    println!();
}

fn list_networks() {
    println!("\n--- Lista Reti Configurate ---");
    
    let _interface = get_interface();
    
    println!("\n📋 CODICE:");
    println!("```rust");
    println!("// Richiedi la lista delle reti");
    println!("stream.write_all(b\"LIST_NETWORKS\\n\")?;");
    println!();
    println!("// Leggi la risposta");
    println!("// Formato: network id / ssid / bssid / flags");
    println!("// 0  MyNetwork  any  [CURRENT]");
    println!("// 1  OtherNet   any  [DISABLED]");
    println!("```");
    println!();
}

fn add_network() {
    println!("\n--- Aggiungi Rete ---");
    
    print!("Inserisci SSID: ");
    io::stdout().flush().unwrap();
    let mut ssid = String::new();
    io::stdin().read_line(&mut ssid).unwrap();
    let ssid = ssid.trim();
    
    print!("Inserisci password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();
    
    println!("\n📋 CODICE:");
    println!("```rust");
    println!("// 1. Aggiungi una nuova rete");
    println!("stream.write_all(b\"ADD_NETWORK\\n\")?;");
    println!("let network_id = read_response()?;  // Es: \"0\"");
    println!();
    println!("// 2. Configura SSID");
    println!("let cmd = format!(\"SET_NETWORK {{}} ssid \\\"{{}}\\\"\\n\", network_id, \"{}\");", ssid);
    println!("stream.write_all(cmd.as_bytes())?;");
    println!();
    if !password.is_empty() {
        println!("// 3. Configura password");
        println!("let cmd = format!(\"SET_NETWORK {{}} psk \\\"{{}}\\\"\\n\", network_id, \"{}\");", "********");
        println!("stream.write_all(cmd.as_bytes())?;");
    } else {
        println!("// 3. Rete aperta");
        println!("let cmd = format!(\"SET_NETWORK {{}} key_mgmt NONE\\n\", network_id);");
        println!("stream.write_all(cmd.as_bytes())?;");
    }
    println!();
    println!("// 4. Abilita la rete");
    println!("let cmd = format!(\"ENABLE_NETWORK {{}}\\n\", network_id);");
    println!("stream.write_all(cmd.as_bytes())?;");
    println!();
    println!("println!(\"✅ Rete aggiunta e abilitata!\");");
    println!("```");
    println!();
}

fn select_network() {
    println!("\n--- Seleziona Rete ---");
    
    print!("Inserisci ID della rete: ");
    io::stdout().flush().unwrap();
    let mut network_id = String::new();
    io::stdin().read_line(&mut network_id).unwrap();
    let network_id = network_id.trim();
    
    println!("\n📋 CODICE:");
    println!("```rust");
    println!("// Seleziona la rete da usare");
    println!("let cmd = format!(\"SELECT_NETWORK {{}}\\n\", {});", network_id);
    println!("stream.write_all(cmd.as_bytes())?;");
    println!();
    println!("// Risposta: \"OK\" se successo");
    println!("```");
    println!();
}

fn disconnect_socket() {
    println!("\n--- Disconnetti Socket ---");
    println!();
    println!("📋 CODICE:");
    println!("```rust");
    println!("// Chiudi la connessione al socket");
    println!("drop(stream);");
    println!("```");
    println!();
    println!("✅ Socket disconnesso!");
}

fn show_implementation_info() {
    println!("\n=== Implementazione Unix Socket ===\n");
    
    println!("📚 PANORAMICA");
    println!("Il control socket di wpa_supplicant permette di inviare comandi testuali");
    println!("direttamente al daemon tramite un Unix Domain Socket.");
    println!("È un'alternativa più semplice a D-Bus ma meno strutturata.");
    println!();
    
    println!("🔧 ARCHITETTURA");
    println!("┌─────────────────┐");
    println!("│ Applicazione    │");
    println!("│ Rust            │");
    println!("└────────┬────────┘");
    println!("         │ Unix Socket");
    println!("         │ (Comandi testuali)");
    println!("         ↓");
    println!("┌─────────────────┐");
    println!("│ wpa_supplicant  │");
    println!("│ Control Socket  │");
    println!("└────────┬────────┘");
    println!("         │");
    println!("         ↓");
    println!("┌─────────────────┐");
    println!("│ Kernel WiFi     │");
    println!("└─────────────────┘");
    println!();
    
    println!("📦 CONFIGURAZIONE WPA_SUPPLICANT");
    println!("Per abilitare il control socket, wpa_supplicant deve essere avviato con:");
    println!("```bash");
    println!("wpa_supplicant -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant.conf");
    println!("```");
    println!();
    println!("E il file di configurazione deve contenere:");
    println!("```");
    println!("ctrl_interface=/var/run/wpa_supplicant");
    println!("ctrl_interface_group=wheel  # o netdev");
    println!("```");
    println!();
    
    println!("🔌 SOCKET PATH");
    println!("Per interfaccia wlan0: /var/run/wpa_supplicant/wlan0");
    println!("Per interfaccia wlan1: /var/run/wpa_supplicant/wlan1");
    println!();
    
    println!("📝 COMANDI PRINCIPALI");
    println!("┌────────────────────┬──────────────────────────────────┐");
    println!("│ Comando            │ Descrizione                      │");
    println!("├────────────────────┼──────────────────────────────────┤");
    println!("│ PING               │ Test connessione (risposta: PONG)│");
    println!("│ SCAN               │ Avvia scansione reti            │");
    println!("│ SCAN_RESULTS       │ Ottieni risultati scansione      │");
    println!("│ STATUS             │ Stato connessione corrente       │");
    println!("│ LIST_NETWORKS      │ Lista reti configurate           │");
    println!("│ ADD_NETWORK        │ Aggiungi nuova rete              │");
    println!("│ SET_NETWORK id ... │ Configura parametri rete         │");
    println!("│ ENABLE_NETWORK id  │ Abilita rete                     │");
    println!("│ SELECT_NETWORK id  │ Connetti a rete                  │");
    println!("│ DISCONNECT         │ Disconnetti                      │");
    println!("│ SAVE_CONFIG        │ Salva configurazione             │");
    println!("└────────────────────┴──────────────────────────────────┘");
    println!();
    
    println!("💻 ESEMPIO COMPLETO");
    println!("```rust");
    println!("use std::os::unix::net::UnixStream;");
    println!("use std::io::{{Write, BufRead, BufReader}};");
    println!();
    println!("fn main() -> std::io::Result<()> {{");
    println!("    // Connetti al socket");
    println!("    let mut stream = UnixStream::connect(\"/var/run/wpa_supplicant/wlan0\")?;");
    println!("    let mut reader = BufReader::new(&stream);");
    println!("    ");
    println!("    // Invia comando PING");
    println!("    stream.write_all(b\"PING\\n\")?;");
    println!("    ");
    println!("    // Leggi risposta");
    println!("    let mut response = String::new();");
    println!("    reader.read_line(&mut response)?;");
    println!("    println!(\"Risposta: {{}}\", response.trim());  // \"PONG\"");
    println!("    ");
    println!("    // Avvia scansione");
    println!("    stream.write_all(b\"SCAN\\n\")?;");
    println!("    response.clear();");
    println!("    reader.read_line(&mut response)?;");
    println!("    println!(\"Scan: {{}}\", response.trim());  // \"OK\"");
    println!("    ");
    println!("    Ok(())");
    println!("}}");
    println!("```");
    println!();
    
    println!("✅ VANTAGGI");
    println!("• Semplice da implementare");
    println!("• Nessuna dipendenza esterna (solo std)");
    println!("• Comunicazione diretta e veloce");
    println!("• Comandi testuali leggibili");
    println!("• Non richiede async runtime");
    println!();
    
    println!("⚠️  SVANTAGGI");
    println!("• Parsing manuale delle risposte testuali");
    println!("• Meno robusto di D-Bus");
    println!("• Gestione eventi più complessa");
    println!("• API non type-safe");
    println!("• Richiede wpa_supplicant configurato con ctrl_interface");
    println!();
    
    println!("🔒 PERMESSI");
    println!("L'utente deve avere accesso al socket, tipicamente tramite:");
    println!("• Essere root");
    println!("• Appartenere al gruppo configurato in ctrl_interface_group");
    println!("  (es: wheel, netdev, network)");
    println!();
    
    println!("🛠️  TOOL UTILI");
    println!("wpa_cli - Client interattivo per il control socket:");
    println!("  wpa_cli -i wlan0");
    println!("  > scan");
    println!("  > scan_results");
    println!("  > status");
    println!();
    
    println!("📖 CONFRONTO CON D-BUS");
    println!("┌────────────────┬──────────────┬──────────────┐");
    println!("│ Caratteristica │ Unix Socket  │ D-Bus        │");
    println!("├────────────────┼──────────────┼──────────────┤");
    println!("│ Complessità    │ Bassa        │ Media/Alta   │");
    println!("│ Type-safety    │ No           │ Sì           │");
    println!("│ Performance    │ Alta         │ Buona        │");
    println!("│ Eventi         │ Manuale      │ Automatico   │");
    println!("│ Dipendenze     │ std only     │ zbus, tokio  │");
    println!("│ Parsing        │ Manuale      │ Automatico   │");
    println!("└────────────────┴──────────────┴──────────────┘");
    println!();
    
    println!("📚 RISORSE");
    println!("• wpa_supplicant ctrl_interface:");
    println!("  https://w1.fi/wpa_supplicant/devel/ctrl_iface_page.html");
    println!("• wpa_cli source code (esempio di client):");
    println!("  https://w1.fi/cgit/hostap/tree/wpa_supplicant/wpa_cli.c");
    println!();
}
