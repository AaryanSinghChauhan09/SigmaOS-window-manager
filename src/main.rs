use sigmaos_window_manager::{Config, WindowManagerComponent, Component};
fn main() {
    env_logger::init();
    let mut c = WindowManagerComponent::new(Config::default());
    if let Err(e) = c.initialize() { eprintln!("Failed: {}", e); std::process::exit(1); }
    println!("SigmaOS window-manager running");
    let _ = c.shutdown();
}
