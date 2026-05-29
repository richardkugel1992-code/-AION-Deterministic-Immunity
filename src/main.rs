use pcap::Capture;
use std::process::Command;

fn main() {
    let my_controller_ip = "192.168.178.50";
    println!("AION-WÄCHTER: Chirurgische Isolation aktiv.");
    let mut cap = Capture::from_device("eth0").unwrap().immediate_mode(true).open().unwrap();

    while let Ok(packet) = cap.next_packet() {
        if packet.header.len > 900 {
            // Logik: IP-Extraktion (Byte 26-29)
            let data = packet.data;
            if data.len() > 30 {
                let ip = format!("{}.{}.{}.{}", data[26], data[27], data[28], data[29]);
                
                // SCHUTZ: Blockiere niemals den Controller (Handy)
                if ip == my_controller_ip { continue; }

                println!("!!! ANOMALIE: IP {} blockiert !!!", ip);
                let _ = Command::new("nft")
                    .args(&["add", "element", "inet", "filter", "blacklist", &format!("{{ {} timeout 1h }}", ip)])
                    .status();
            }
        }
    }
}
