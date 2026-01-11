# WiFi Connector - Terminal App per Linux

Un'applicazione educativa terminal-based in Rust per connettersi alle reti WiFi su distribuzioni Linux.

## Scopo

Questo progetto dimostra diversi approcci per implementare la gestione delle connessioni WiFi su Linux, con focus educativo per comprendere le varie alternative architetturali.

## Implementazioni Disponibili

### 1. NetworkManager (nmcli) - `wifi-connector`

Implementazione ad alto livello usando NetworkManager.

**Requisiti:**
- NetworkManager installato
- nmcli disponibile nel PATH

**Esecuzione:**
```bash
cargo run --bin wifi-connector
# oppure
cargo run
```

### 2. wpa_supplicant - `wifi-wpa-supplicant` ✨ NUOVO

Implementazione a basso livello usando wpa_supplicant direttamente.

**Requisiti:**
- wpasupplicant
- iw (wireless tools)
- isc-dhcp-client o dhcpcd
- Privilegi root per alcune operazioni

**Installazione dipendenze (Debian/Ubuntu):**
```bash
sudo apt install wpasupplicant iw isc-dhcp-client
```

**Esecuzione:**
```bash
# Senza privilegi (modalità demo, alcune operazioni falliranno)
cargo run --bin wifi-wpa-supplicant

# Con privilegi root (funzionalità complete)
sudo cargo run --bin wifi-wpa-supplicant
```

## Funzionalità

### Implementazione NetworkManager
- 🔍 Scansione reti WiFi disponibili
- 🔗 Connessione a reti WiFi (con o senza password)
- 📋 Visualizzazione reti salvate
- ❌ Disconnessione dalla rete corrente
- 📊 Stato della connessione
- 📚 Informazioni dettagliate sugli approcci implementativi

### Implementazione wpa_supplicant
- 🔍 Scansione reti WiFi (usando iw scan)
- 📝 Generazione configurazione wpa_supplicant
- 🔗 Connessione usando wpa_supplicant
- 🌐 Ottenimento indirizzo IP via DHCP
- ❌ Disconnessione e pulizia
- 📊 Stato wpa_supplicant e interfaccia
- 📚 Documentazione dettagliata del workflow

## Installazione

```bash
# Clona o naviga nella directory del progetto
cd wifi-connector

# Compila entrambe le implementazioni
cargo build --release

# Esegui l'implementazione desiderata
cargo run --bin wifi-connector          # NetworkManager
cargo run --bin wifi-wpa-supplicant     # wpa_supplicant
```

## Utilizzo

### NetworkManager (wifi-connector)

L'applicazione presenta un menu interattivo:

```
=== WiFi Connector per Linux ===

--- Menu Principale ---
1. Scansiona reti WiFi disponibili
2. Connetti a una rete WiFi
3. Mostra reti salvate
4. Disconnetti dalla rete corrente
5. Mostra stato connessione
6. Info sugli approcci implementativi
0. Esci
```

### wpa_supplicant (wifi-wpa-supplicant)

```
=== WiFi Connector con wpa_supplicant ===

--- Menu wpa_supplicant ---
1. Scansiona reti WiFi (usando iw scan)
2. Genera configurazione wpa_supplicant
3. Connetti usando wpa_supplicant
4. Ottieni indirizzo IP (DHCP)
5. Disconnetti
6. Mostra stato wpa_supplicant
7. Info su questa implementazione
0. Esci
```

**Workflow tipico con wpa_supplicant:**
1. Opzione 1: Scansiona le reti disponibili
2. Opzione 2: Genera il file di configurazione per la rete scelta
3. Opzione 3: Avvia wpa_supplicant per connettersi
4. Opzione 4: Ottieni un indirizzo IP via DHCP
5. Opzione 6: Verifica lo stato della connessione

### Esempi di Utilizzo

**Scansionare le reti:**
```
Scegli un'opzione: 1
```

**Connettersi a una rete:**
```
Scegli un'opzione: 2
Inserisci SSID della rete: MiaReteWiFi
Inserisci password: miapassword123
```

**Vedere lo stato della connessione:**
```
Scegli un'opzione: 5
```

## Approcci Implementativi

### 1. NetworkManager (nmcli) ✅ IMPLEMENTATO

**File:** `src/main.rs` | **Binary:** `wifi-connector`

**Implementazione:** Utilizza il comando `nmcli` per interfacciarsi con NetworkManager.

**Vantaggi:**
- Alto livello, facile da usare
- Gestione automatica della configurazione
- Ampio supporto nelle distribuzioni moderne

**Come funziona:**
```rust
Command::new("nmcli")
    .args(&["device", "wifi", "connect", ssid, "password", password])
    .output()
```

### 2. WPA_Supplicant ✅ IMPLEMENTATO

**File:** `src/wpa_supplicant.rs` | **Binary:** `wifi-wpa-supplicant`

Approccio più a basso livello che interagisce direttamente con `wpa_supplicant`.

**Implementazione:**
1. Scansione con `iw <interface> scan`
2. Creazione file di configurazione wpa_supplicant
3. Avvio: `wpa_supplicant -B -i <interface> -c <config>`
4. DHCP: `dhclient <interface>`

**Vantaggi:**
- Maggior controllo sul processo
- Non dipende da NetworkManager
- Funziona su sistemi minimali/embedded
- Ideale per server headless

**Svantaggi:**
- Più complesso
- Richiede privilegi root
- Configurazione manuale separata per autenticazione e DHCP

**Workflow completo:**
```rust
// 1. Scan
Command::new("iw").args(&[interface, "scan"]).output()

// 2. Generate config
let config = format!("network={{\n  ssid=\"{}\"\n  psk=\"{}\"\n}}", ssid, password);
fs::write(config_file, config)

// 3. Start wpa_supplicant
Command::new("wpa_supplicant")
    .args(&["-B", "-i", interface, "-c", config_file])
    .output()

// 4. Get IP
Command::new("dhclient").arg(interface).output()
```

### 3. IWD (iNet Wireless Daemon) 📝 TEORICO

Alternativa moderna a wpa_supplicant.

**Come si implementerebbe:**
```bash
iwctl station wlan0 connect "SSID"
```

**Vantaggi:**
- Moderno e performante
- Configurazione più semplice

**Svantaggi:**
- Meno diffuso
- Meno documentazione

### 4. Librerie Rust Native 📝 TEORICO

Utilizzo di crate Rust per accesso diretto.

**Crate potenziali:**
- `network-manager` - Bindings per D-Bus API di NetworkManager
- `libc` - Accesso a basso livello (socket, ioctl)
- `neli` - Comunicazione Netlink

**Vantaggi:**
- Type-safe
- Nessuna dipendenza da comandi esterni
- Migliore error handling

**Svantaggi:**
- Implementazione complessa
- Richiede conoscenze a basso livello

## Struttura del Codice

```
wifi-connector/
├── Cargo.toml          # Configurazione del progetto
├── README.md           # Questa documentazione
└── src/
    └── main.rs         # Codice principale con menu e funzionalità
```

### Strutture Dati Principali

```rust
// Enum per rappresentare diversi approcci
enum WifiApproach {
    NetworkManager,
    WpaSupplicant,
    IwdCtl,
}

// Struttura per una rete WiFi
struct WifiNetwork {
    ssid: String,
    signal: String,
    security: String,
}
```

## Considerazioni di Sicurezza ⚠️

**IMPORTANTE:** Questa è un'applicazione educativa!

- Le password vengono passate come argomenti da riga di comando (visibili in `ps`)
- Non adatta per uso in produzione
- Per applicazioni reali, considerare:
  - Uso di D-Bus API invece di nmcli
  - Keyring del sistema per le password
  - File di configurazione protetti

## Permessi

Alcune operazioni potrebbero richiedere privilegi elevati. NetworkManager di solito gestisce i permessi tramite PolicyKit.

Se riscontri errori di permessi:
```bash
# Aggiungi il tuo utente al gruppo necessario
sudo usermod -aG netdev $USER
```

## Apprendimento

Questo progetto è ideale per:
- Comprendere diversi approcci architetturali
- Imparare l'interazione con comandi di sistema in Rust
- Esplorare le API di rete di Linux
- Praticare il parsing di output testuale
- Capire la gestione delle connessioni WiFi su Linux

## Limitazioni

- Implementato solo l'approccio NetworkManager
- Non gestisce reti enterprise (802.1X)
- Password in chiaro negli argomenti del comando
- Limitato a distribuzioni Linux con NetworkManager

## Sviluppi Futuri

Possibili estensioni educative:
- [ ] Implementare approccio WPA_Supplicant
- [ ] Aggiungere supporto per IWD
- [ ] Usare crate network-manager per accesso D-Bus
- [ ] Gestire reti enterprise
- [ ] Aggiungere hotspot creation
- [ ] TUI con crate come `tui-rs` o `cursive`

## Risorse

- [NetworkManager Documentation](https://networkmanager.dev/)
- [nmcli Examples](https://developer-old.gnome.org/NetworkManager/stable/nmcli-examples.html)
- [WPA Supplicant](https://w1.fi/wpa_supplicant/)
- [IWD Project](https://iwd.wiki.kernel.org/)

## Licenza

Questo progetto fa parte del repository rust-learning ed è rilasciato sotto la stessa licenza del repository principale.

## Autore

Progetto educativo per il repository rust-learning.
