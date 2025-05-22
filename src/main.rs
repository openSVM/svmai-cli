// src/main.rs
mod file_searcher;
mod key_validator;
mod secure_storage;
mod tui;
mod wallet_manager;
mod vanity_wallet;

fn main() -> std::io::Result<()> {
    // For now, directly launch the TUI.
    // Later, we can add CLI arguments to select modes (e.g., scan, TUI, specific commands).
    println!("Starting svmai tool in TUI mode...");
    tui::run_tui()
}
