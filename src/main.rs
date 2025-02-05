use tray_item::{TrayItem, IconSource};
use battery::{Manager, State};
use std::io;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

enum BatteryUpdate {
    Update { icon: &'static str},
}


fn main() {
    // Initialize GTK.
    gtk::init().unwrap();

    // Create the tray icon with an initial icon ("battery-full") and a label.
    let mut tray = TrayItem::new("Waybat", IconSource::Resource("battery-full"))
        .expect("Failed to create tray icon");
    tray.add_label("Waybat").expect("Failed to add label");
    tray.add_menu_item("Quit", || {
        gtk::main_quit();
    }).expect("Failed to add Quit menu item");

    // Create a channel to send battery update messages.
    let (tx, rx) = mpsc::channel::<BatteryUpdate>();

    // Spawn a thread that monitors the battery status and sends updates.
    thread::spawn(move || {
        if let Err(e) = battery_loop(tx) {
            eprintln!("Error in battery_loop: {:?}", e);
        }
    });

    // In the main thread, use glib's timeout to poll the channel periodically
    // and update the tray icon.
    gtk::glib::timeout_add_local(Duration::from_millis(100), move || {
        // Process all available messages.
        while let Ok(update) = rx.try_recv() {
            match update {
                BatteryUpdate::Update { icon } => {
                    // Update the tray icon. We use IconSource::Resource here.
                    if let Err(e) = tray.set_icon(IconSource::Resource(icon)) {

                        eprintln!("Error updating icon: {:?}", e);
                    }
                }
            }
        }
        // Return true to continue polling.
        true.into()
    });

    // Start the GTK main loop.
    gtk::main();
}

/// battery_loop reads the battery status and sends updates via the provided channel.
fn battery_loop(tx: Sender<BatteryUpdate>) -> battery::Result<()> {
    let manager = Manager::new()?;
    // Try to get the first available battery.
    let mut battery = match manager.batteries()?.next() {
        Some(Ok(bat)) => bat,
        Some(Err(e)) => {
            eprintln!("Unable to access battery information");
            return Err(e);
        },
        None => {
            eprintln!("Unable to find any batteries");
            return Err(io::Error::from(io::ErrorKind::NotFound).into());
        }
    };

    loop {
        // Calculate the battery charge percentage.
        let charge = battery.state_of_charge().value * 100.0;
        println!("Battery level: {:.1}%", charge);

        // Determine the icon based on the battery level and state.
        let icon = if battery.state() == State::Charging && charge < 20.0 {
            "battery-empty-charging"
        } else if battery.state() == State::Charging && charge < 50.0 {
            "battery-caution-charging"
        } else if battery.state() == State::Charging && charge < 80.0 {
            "battery-good-charging"
        } else if battery.state() == State::Charging && charge > 90.0 {
            "battery-full-charging"
        } else if battery.state() != State::Charging && charge < 20.0 {
            "battery-empty"
        } else if battery.state() != State::Charging && charge < 50.0 {
            "battery-level-40"
        } else if battery.state() != State::Charging && charge < 80.0 {
            "battery-good"
        } else {
            "battery-full"
        };

        // Send an update message via the channel.
        tx.send(BatteryUpdate::Update { icon }).unwrap();


        // Wait 15 seconds before checking again.
        thread::sleep(Duration::from_secs(15));
        // Refresh the battery information.
        manager.refresh(&mut battery)?;
    }
}

