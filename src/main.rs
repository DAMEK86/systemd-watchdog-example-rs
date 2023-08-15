use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("check for systemd");
    sd_notify::booted().expect("systemd init system required");
    let mut usec = 0;
    let watchdog_enabled = sd_notify::watchdog_enabled(false, &mut usec);
    if !watchdog_enabled {
        panic!("executable only works with systemd `Type=notify` and `WatchdogSec=x`")
    }
    let watchdog_delay = Duration::from_micros(usec/2);
    println!("watchdog enabled, trigger watchdog every {} seconds", watchdog_delay.as_secs());
    sd_notify::notify(false, &[sd_notify::NotifyState::Ready]).expect("service need to up");
    loop {
        sleep(watchdog_delay);
        sd_notify::notify(false, &[sd_notify::NotifyState::Watchdog]).expect("watchdog needs to be send.")
    }
}
