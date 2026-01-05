# WiFi Connector - Terminal App per Linux

Un'applicazione educativa terminal-based in Rust per connettersi alle reti WiFi su distribuzioni Linux.

## Scopo

Questo progetto dimostra diversi approcci per implementare la gestione delle connessioni WiFi su Linux, con focus educativo per comprendere le varie alternative architetturali.

## Funzionalità

- 🔍 Scansione reti WiFi disponibili
- 🔗 Connessione a reti WiFi (con o senza password)
- 📋 Visualizzazione reti salvate
- ❌ Disconnessione dalla rete corrente
- 📊 Stato della connessione
- 📚 Informazioni dettagliate sugli approcci implementativi

## Requisiti

- **Rust** (edition 2021 o superiore)
- **NetworkManager** installato nel sistema
- **nmcli** disponibile nel PATH
- Distribuzioni Linux supportate: Ubuntu, Fedora, Arch, Debian, ecc.

## Installazione

```bash
# Clona o naviga nella directory del progetto
cd wifi-connector

# Compila il progetto
cargo build --release

# Esegui l'applicazione
cargo run
```

## Utilizzo

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

**Implementazione attuale:** Utilizza il comando `nmcli` per interfacciarsi con NetworkManager.

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

### 2. WPA_Supplicant 📝 TEORICO

Approccio più a basso livello che interagisce direttamente con `wpa_supplicant`.

**Come si implementerebbe:**
- Creare file di configurazione `/etc/wpa_supplicant/wpa_supplicant.conf`
- Eseguire `wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant/wpa_supplicant.conf`
- Gestire DHCP con `dhclient`

**Vantaggi:**
- Maggior controllo
- Non dipende da NetworkManager
- Funziona su sistemi minimali

**Svantaggi:**
- Più complesso
- Richiede privilegi root
- Configurazione manuale

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
