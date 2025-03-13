#![windows_subsystem="windows"]
use std::time::{Duration};
use sysinfo::{NetworkExt, System, SystemExt};
use humansize::{format_size, DECIMAL};
use tray_icon::{Icon, TrayIconBuilder};

enum ThemeIcon { Fav, Up, Down, Off }

impl ThemeIcon {
    fn resource(&self) -> Icon {
        match self {
            Self::Fav => Icon::from_resource_name("fav", Some((32,32))).expect("message"),
            Self::Up => Icon::from_resource_name("up", Some((32,32))).expect("message"),
            Self::Down => Icon::from_resource_name("down", Some((32,32))).expect("message"),
            Self::Off => Icon::from_resource_name("off", Some((32,32))).expect("message"),
        }
    }
}

fn main() {
    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("Net Widget")
        .with_icon(ThemeIcon::Fav.resource())
        .build()
        .unwrap();
    
    let mut sys: System = System::new_all();
    sys.refresh_all();


    println!("Network Speed Monitor");

    loop {
        sys.refresh_all();

        let networks = sys.networks();
        let mut max_interface = None;
        let mut max_total = 0;

        for (name, network) in networks {
            let total = network.received() + network.transmitted();
            if total > max_total {
                max_total = total;
                max_interface = Some((name, network));
            }
        }

        
        if let Some((_name, network)) = max_interface {
            let total_rx = network.received();
            let total_tx = network.transmitted();

            let rx_str = format_size(total_rx , DECIMAL);
            let tx_str = format_size(total_tx , DECIMAL);
            
            // println!("Network Speed ({}):", name);
            // println!("  ↓ {}/s", rx_str);
            // println!("  ↑ {}/s", tx_str);
            // println!("----------------------------------------");

            let label = format!("↓ {}/s \n↑ {}/s", rx_str, tx_str);
            tray_icon.set_tooltip(Some(&label)).unwrap();

            if total_rx > total_tx {
                tray_icon.set_icon(Some(ThemeIcon::Down.resource())).expect("Failed to Down Icon");
            }
            else{
                tray_icon.set_icon(Some(ThemeIcon::Up.resource())).expect("Failed to Up Icon");
            }
        }
        else {
            tray_icon.set_icon(Some(ThemeIcon::Off.resource())).expect("Failed to Off Icon");
            tray_icon.set_tooltip(Some("Offline!")).unwrap();
        }
        
        // print!("\x1B[2J\x1B[1;1H"); 
        std::thread::sleep(Duration::from_secs(1));
    }
}
