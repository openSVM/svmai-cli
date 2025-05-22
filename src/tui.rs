// src/tui.rs

// This module handles the Text-based User Interface (TUI)
// using ratatui and crossterm.

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event as CrosstermEvent, KeyCode,
        KeyEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use std::io::{self, stdout, Stdout};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use crate::secure_storage;
use crate::wallet_manager; // To interact with wallet data
use crate::vanity_wallet::{self, VanityConfig, VanityStatus}; // For vanity wallet creation

// Define different views for the TUI
enum View {
    WalletList,
    WalletDetail,
    Help,
    AddWallet,
    ConfirmDelete,
    SearchWallets,
    BatchOperations,
    CreateVanityWallet,
    VanityProgress,
}

// Define possible status messages
enum StatusType {
    Info,
    Success,
    Error,
    Warning,
}

struct StatusMessage {
    message: String,
    status_type: StatusType,
    timestamp: Instant,
}

// Application state
struct App {
    wallets: Vec<String>,           // List of wallet names
    wallet_details: Vec<WalletDetail>, // Details for each wallet
    selected_wallet: Option<usize>, // Index of the selected wallet
    should_quit: bool,
    current_view: View,
    status_message: Option<StatusMessage>,
    input_buffer: String,           // For text input in add wallet view
    confirm_action: bool,           // For confirmation dialogs
    search_query: String,           // For wallet search functionality
    filtered_wallets: Vec<usize>,   // Indices of wallets matching search
    scroll_offset: usize,           // For scrolling in long lists
    last_refresh: Instant,          // Track when wallet data was last refreshed
    vanity_config: VanityConfig,    // Configuration for vanity wallet generation
    vanity_status: Option<VanityStatus>, // Status of vanity wallet generation
    vanity_cancelled: Arc<AtomicBool>, // Flag to cancel vanity generation
    vanity_wallet_name: String,     // Name for the new vanity wallet
    vanity_thread: Option<thread::JoinHandle<()>>, // Handle to vanity generation thread
    vanity_result: Arc<Mutex<Option<solana_sdk::signer::keypair::Keypair>>>, // Result of vanity generation
}

// Wallet detail information
struct WalletDetail {
    name: String,
    pubkey: Option<Pubkey>,
    balance: Option<f64>,
    last_transaction: Option<String>,
    token_balances: Vec<TokenBalance>, // Added for SPL token balances
}

// Structure to hold token balance information
struct TokenBalance {
    token_name: String,
    mint_address: String,
    amount: f64,
}

impl App {
    fn new() -> Self {
        App {
            wallets: Vec::new(),
            wallet_details: Vec::new(),
            selected_wallet: None,
            should_quit: false,
            current_view: View::WalletList,
            status_message: None,
            input_buffer: String::new(),
            confirm_action: false,
            search_query: String::new(),
            filtered_wallets: Vec::new(),
            scroll_offset: 0,
            last_refresh: Instant::now(),
            vanity_config: VanityConfig {
                prefix: "ai".to_string(), // Default prefix as requested
                timeout_seconds: 120,     // 2 minutes default timeout
                thread_count: num_cpus::get().min(8), // Use up to 8 threads to avoid excessive CPU usage
                progress_interval_ms: 250, // More frequent updates for responsive UI
            },
            vanity_status: None,
            vanity_cancelled: Arc::new(AtomicBool::new(false)),
            vanity_wallet_name: "ai_wallet".to_string(),
            vanity_thread: None,
            vanity_result: Arc::new(Mutex::new(None)),
        }
    }

    fn load_wallets(&mut self) {
        match secure_storage::list_wallet_names() {
            Ok(names) => {
                self.wallets = names;
                self.load_wallet_details();
                self.last_refresh = Instant::now();
                self.update_filtered_wallets();
            },
            Err(e) => {
                self.set_status(format!("Error loading wallets: {}", e), StatusType::Error);
                self.wallets = Vec::new();
            }
        }
        
        if !self.wallets.is_empty() && self.selected_wallet.is_none() {
            self.selected_wallet = Some(0);
        }
    }
    
    fn load_wallet_details(&mut self) {
        self.wallet_details.clear();
        let mut error_messages = Vec::new();
        
        for wallet_name in &self.wallets.clone() {
            let mut detail = WalletDetail {
                name: wallet_name.clone(),
                pubkey: None,
                balance: None,
                last_transaction: None,
                token_balances: Vec::new(), // Initialize empty token balances
            };
            
            // Try to get the keypair to extract public key
            match wallet_manager::get_wallet_keypair(wallet_name) {
                Ok(Some(keypair)) => {
                    detail.pubkey = Some(keypair.pubkey());
                    // In a real implementation, we would fetch balance and transaction history
                    // For now, we'll use placeholder values
                    detail.balance = Some(0.0); // Placeholder
                    detail.last_transaction = Some("No transactions yet".to_string());
                    
                    // Add some example token balances for demonstration
                    detail.token_balances.push(TokenBalance {
                        token_name: "USDC".to_string(),
                        mint_address: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
                        amount: 100.0,
                    });
                    detail.token_balances.push(TokenBalance {
                        token_name: "RAY".to_string(),
                        mint_address: "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R".to_string(),
                        amount: 25.5,
                    });
                },
                Ok(None) => {
                    // Wallet exists but couldn't get keypair
                    detail.pubkey = None;
                },
                Err(e) => {
                    error_messages.push(format!("Error loading wallet details for {}: {}", wallet_name, e));
                }
            }
            
            self.wallet_details.push(detail);
        }
        
        // Set status message if there were errors
        if !error_messages.is_empty() {
            self.set_status(
                format!("Errors loading some wallet details: {}", error_messages.join("; ")), 
                StatusType::Error
            );
        }
    }
    
    fn set_status(&mut self, message: String, status_type: StatusType) {
        self.status_message = Some(StatusMessage {
            message,
            status_type,
            timestamp: Instant::now(),
        });
    }
    
    fn clear_status_if_expired(&mut self) {
        if let Some(status) = &self.status_message {
            if status.timestamp.elapsed() > Duration::from_secs(5) {
                self.status_message = None;
            }
        }
    }
    
    fn add_wallet(&mut self, file_path: String) {
        if file_path.is_empty() {
            self.set_status("Please enter a valid file path".to_string(), StatusType::Warning);
            return;
        }
        
        // Generate a wallet name from the file path
        let file_name = match std::path::Path::new(&file_path).file_stem() {
            Some(stem) => stem.to_string_lossy().to_string(),
            None => "new_wallet".to_string(),
        };
        
        match wallet_manager::add_wallet_from_file(&file_name, &file_path) {
            Ok(_) => {
                self.set_status(format!("Wallet '{}' added successfully", file_name), StatusType::Success);
                self.load_wallets(); // Refresh wallet list
                self.current_view = View::WalletList;
                self.input_buffer.clear();
            },
            Err(e) => {
                self.set_status(format!("Failed to add wallet: {}", e), StatusType::Error);
            }
        }
    }
    
    fn remove_selected_wallet(&mut self) {
        if let Some(selected) = self.selected_wallet {
            if selected < self.wallets.len() {
                let wallet_name = &self.wallets[selected];
                match wallet_manager::remove_wallet(wallet_name) {
                    Ok(_) => {
                        self.set_status(format!("Wallet '{}' removed successfully", wallet_name), StatusType::Success);
                        self.load_wallets(); // Refresh wallet list
                        
                        // Adjust selected index if needed
                        if self.wallets.is_empty() {
                            self.selected_wallet = None;
                        } else if selected >= self.wallets.len() {
                            self.selected_wallet = Some(self.wallets.len() - 1);
                        }
                    },
                    Err(e) => {
                        self.set_status(format!("Failed to remove wallet: {}", e), StatusType::Error);
                    }
                }
            }
        }
        self.current_view = View::WalletList;
        self.confirm_action = false;
    }
    
    fn update_filtered_wallets(&mut self) {
        if self.search_query.is_empty() {
            // If no search query, include all wallets
            self.filtered_wallets = (0..self.wallets.len()).collect();
        } else {
            // Filter wallets based on search query (case-insensitive)
            let query = self.search_query.to_lowercase();
            self.filtered_wallets = self.wallets.iter()
                .enumerate()
                .filter(|(_, name)| name.to_lowercase().contains(&query))
                .map(|(i, _)| i)
                .collect();
        }
        
        // Reset selection if current selection is not in filtered list
        if let Some(selected) = self.selected_wallet {
            if !self.filtered_wallets.contains(&selected) {
                self.selected_wallet = self.filtered_wallets.first().copied();
            }
        } else if !self.filtered_wallets.is_empty() {
            self.selected_wallet = Some(self.filtered_wallets[0]);
        }
    }
    
    fn select_next_wallet(&mut self) {
        if self.filtered_wallets.is_empty() {
            return;
        }
        
        if let Some(selected) = self.selected_wallet {
            // Find the current position in filtered wallets
            if let Some(pos) = self.filtered_wallets.iter().position(|&i| i == selected) {
                if pos < self.filtered_wallets.len() - 1 {
                    self.selected_wallet = Some(self.filtered_wallets[pos + 1]);
                }
            } else if !self.filtered_wallets.is_empty() {
                self.selected_wallet = Some(self.filtered_wallets[0]);
            }
        } else if !self.filtered_wallets.is_empty() {
            self.selected_wallet = Some(self.filtered_wallets[0]);
        }
    }
    
    fn select_prev_wallet(&mut self) {
        if self.filtered_wallets.is_empty() {
            return;
        }
        
        if let Some(selected) = self.selected_wallet {
            // Find the current position in filtered wallets
            if let Some(pos) = self.filtered_wallets.iter().position(|&i| i == selected) {
                if pos > 0 {
                    self.selected_wallet = Some(self.filtered_wallets[pos - 1]);
                }
            } else if !self.filtered_wallets.is_empty() {
                self.selected_wallet = Some(self.filtered_wallets[0]);
            }
        } else if !self.filtered_wallets.is_empty() {
            self.selected_wallet = Some(self.filtered_wallets[0]);
        }
    }
    
    fn start_vanity_wallet_creation(&mut self) {
        // Reset status
        self.vanity_status = Some(VanityStatus {
            attempts: 0,
            completed: false,
            success: false,
            pubkey: None,
            elapsed_seconds: 0.0,
            attempts_per_second: 0.0,
        });
        
        // Reset cancellation flag
        self.vanity_cancelled.store(false, Ordering::SeqCst);
        
        // Reset result
        let mut result = self.vanity_result.lock().unwrap();
        *result = None;
        drop(result);
        
        // Create shared status for thread communication
        let status = Arc::new(Mutex::new(self.vanity_status.clone().unwrap()));
        let status_for_ui = Arc::clone(&status);
        
        // Start vanity wallet generation in a separate thread
        let vanity_config = self.vanity_config.clone();
        let cancelled = Arc::clone(&self.vanity_cancelled);
        let result = Arc::clone(&self.vanity_result);
        
        let handle = thread::spawn(move || {
            // Create a local callback that updates the shared status
            let status_clone = Arc::clone(&status);
            let keypair_result = vanity_wallet::generate_vanity_keypair_with_progress(
                &vanity_config,
                move |new_status| {
                    let mut status_guard = status_clone.lock().unwrap();
                    *status_guard = new_status.clone();
                }
            );
            
            // Store final result
            if let Ok(keypair) = keypair_result {
                let mut result_guard = result.lock().unwrap();
                *result_guard = Some(keypair);
            }
        });
        
        // Store thread handle and status for UI updates
        self.vanity_thread = Some(handle);
        self.vanity_status = Some(status_for_ui.lock().unwrap().clone());
        
        // Switch to progress view
        self.current_view = View::VanityProgress;
        
        // Set initial status message
        self.set_status(
            format!("Generating vanity wallet with prefix '{}'...", self.vanity_config.prefix),
            StatusType::Info
        );
    }
    
    fn update_vanity_status(&mut self) {
        // Get the latest status from the shared status
        let status_updated = {
            // Check if vanity generation is complete by checking the result
            let result_guard = self.vanity_result.lock().unwrap();
            if result_guard.is_some() {
                // Vanity generation succeeded, extract the keypair
                if let Some(keypair) = &*result_guard {
                    // Create a copy of the keypair to avoid borrow issues
                    let keypair_bytes = keypair.to_bytes();
                    drop(result_guard); // Release the lock before calling save_vanity_wallet
                    
                    // Reconstruct the keypair from bytes
                    if let Ok(keypair_copy) = solana_sdk::signer::keypair::Keypair::from_bytes(&keypair_bytes) {
                        self.save_vanity_wallet(&keypair_copy);
                    } else {
                        self.set_status(
                            "Failed to process vanity wallet keypair".to_string(),
                            StatusType::Error
                        );
                    }
                }
                true
            } else {
                // Update the status from the shared status
                false
            }
        };
        
        if !status_updated {
            // If we didn't find a result, check if the process is still running
            if let Some(thread) = &self.vanity_thread {
                if thread.is_finished() {
                    // Thread is done but no result, must have timed out or been cancelled
                    self.current_view = View::WalletList;
                    self.set_status(
                        "Vanity wallet generation completed without finding a match".to_string(),
                        StatusType::Warning
                    );
                }
            }
        }
    }
    
    fn save_vanity_wallet(&mut self, keypair: &solana_sdk::signer::keypair::Keypair) {
        // Save the keypair to a temporary file
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join(format!("{}.json", self.vanity_wallet_name));
        
        // Convert keypair to bytes and save as JSON array
        let key_bytes = keypair.to_bytes();
        let json_array = serde_json::to_string(&key_bytes.to_vec()).unwrap_or_default();
        
        match std::fs::write(&file_path, json_array) {
            Ok(_) => {
                // Add wallet from the temporary file
                match wallet_manager::add_wallet_from_file(&self.vanity_wallet_name, file_path.to_str().unwrap()) {
                    Ok(_) => {
                        self.set_status(
                            format!("Vanity wallet '{}' created successfully with address {}", 
                                   self.vanity_wallet_name, keypair.pubkey()),
                            StatusType::Success
                        );
                        self.load_wallets(); // Refresh wallet list
                        self.current_view = View::WalletList;
                    },
                    Err(e) => {
                        self.set_status(
                            format!("Failed to add vanity wallet: {}", e),
                            StatusType::Error
                        );
                    }
                }
                
                // Clean up temporary file
                let _ = std::fs::remove_file(file_path);
            },
            Err(e) => {
                self.set_status(
                    format!("Failed to save vanity wallet: {}", e),
                    StatusType::Error
                );
            }
        }
    }
    
    fn cancel_vanity_generation(&mut self) {
        // Use the vanity_cancelled Arc to signal cancellation
        vanity_wallet::cancel_vanity_generation(&self.vanity_cancelled);
        
        self.set_status("Vanity wallet generation cancelled".to_string(), StatusType::Warning);
        self.current_view = View::WalletList;
    }
}

// Main TUI run function
pub fn run_tui() -> io::Result<()> {
    let mut terminal = init_terminal()?;
    let mut app = App::new();
    app.load_wallets(); // Load initial wallet list
    app.set_status("Welcome to svmai wallet manager".to_string(), StatusType::Info);

    loop {
        app.clear_status_if_expired();
        
        // Update vanity status if in progress
        if let View::VanityProgress = app.current_view {
            app.update_vanity_status();
        }
        
        terminal.draw(|frame| ui(frame, &mut app))?;

        if event::poll(Duration::from_millis(100))? {
            if let CrosstermEvent::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    handle_key_event(&mut app, key.code);
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    restore_terminal()?;
    Ok(())
}

// Initialize the terminal
fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

// Restore the terminal to its original state
fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

// Define the UI layout and widgets
fn ui(frame: &mut Frame, app: &mut App) {
    // Create the main layout
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title bar
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Status bar
        ])
        .split(frame.size());

    // Render title with app version and current time
    let title = match app.current_view {
        View::WalletList => "Wallet List",
        View::WalletDetail => "Wallet Details",
        View::Help => "Help",
        View::AddWallet => "Add New Wallet",
        View::ConfirmDelete => "Confirm Delete",
        View::SearchWallets => "Search Wallets",
        View::BatchOperations => "Batch Operations",
        View::CreateVanityWallet => "Create Vanity Wallet",
        View::VanityProgress => "Generating Vanity Wallet",
    };
    
    // Format last refresh time
    let refresh_time = format!("Last refresh: {} seconds ago", 
                              app.last_refresh.elapsed().as_secs());
    
    let title_text = Line::from(vec![
        Span::styled("svmai ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled("v1.0.0 ", Style::default().fg(Color::Gray)),
        Span::styled("| ", Style::default().fg(Color::DarkGray)),
        Span::styled(title, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(" | ", Style::default().fg(Color::DarkGray)),
        Span::styled(refresh_time, Style::default().fg(Color::Gray)),
    ]);
    
    frame.render_widget(
        Paragraph::new(title_text)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL)),
        main_layout[0],
    );

    // Render main content based on current view
    match app.current_view {
        View::WalletList => render_wallet_list(frame, app, main_layout[1]),
        View::WalletDetail => render_wallet_detail(frame, app, main_layout[1]),
        View::Help => render_help(frame, main_layout[1]),
        View::AddWallet => render_add_wallet(frame, app, main_layout[1]),
        View::ConfirmDelete => render_confirm_delete(frame, app, main_layout[1]),
        View::SearchWallets => render_search_wallets(frame, app, main_layout[1]),
        View::BatchOperations => render_batch_operations(frame, app, main_layout[1]),
        View::CreateVanityWallet => render_create_vanity_wallet(frame, app, main_layout[1]),
        View::VanityProgress => render_vanity_progress(frame, app, main_layout[1]),
    }

    // Render status bar
    render_status_bar(frame, app, main_layout[2]);
}

fn render_wallet_list(frame: &mut Frame, app: &App, area: Rect) {
    // Create a layout with search bar at top if needed
    let list_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search bar or stats
            Constraint::Min(0),    // Wallet list
        ])
        .split(area);
    
    // Render wallet stats or search bar
    let wallet_count = app.wallets.len();
    let filtered_count = app.filtered_wallets.len();
    let stats_text = if app.search_query.is_empty() {
        format!("Total wallets: {}", wallet_count)
    } else {
        format!("Showing {} of {} wallets matching: \"{}\"", 
                filtered_count, wallet_count, app.search_query)
    };
    
    frame.render_widget(
        Paragraph::new(stats_text)
            .alignment(Alignment::Left)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Wallet Stats")
                .title_alignment(Alignment::Center)),
        list_layout[0],
    );

    if app.filtered_wallets.is_empty() {
        let message = if app.wallets.is_empty() {
            "No wallets found. Press 'a' to add a wallet or 'v' to create a vanity wallet."
        } else {
            "No wallets match your search criteria."
        };
        
        frame.render_widget(
            Paragraph::new(message)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL)),
            list_layout[1],
        );
        return;
    }

    // Create list items from filtered wallets
    let items: Vec<ListItem> = app.filtered_wallets.iter()
        .map(|&index| {
            let wallet_name = &app.wallets[index];
            let mut style = Style::default();
            if app.selected_wallet == Some(index) {
                style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
            }
            
            // Get public key if available
            let pubkey_display = if index < app.wallet_details.len() {
                if let Some(pubkey) = &app.wallet_details[index].pubkey {
                    let pubkey_str = pubkey.to_string();
                    format!(" ({}...{})", 
                        &pubkey_str[..4], 
                        &pubkey_str[pubkey_str.len()-4..])
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            };
            
            // Get balance if available
            let balance_display = if index < app.wallet_details.len() {
                if let Some(balance) = app.wallet_details[index].balance {
                    format!(" | {:.4} SOL", balance)
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            };
            
            ListItem::new(Line::from(vec![
                Span::styled(format!("{}", wallet_name), style),
                Span::styled(pubkey_display, Style::default().fg(Color::DarkGray)),
                Span::styled(balance_display, Style::default().fg(Color::Green)),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Wallets")
                .title_alignment(Alignment::Center)
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // Create a stateful list
    let mut state = ListState::default();
    
    // Find the position of the selected wallet in the filtered list
    if let Some(selected) = app.selected_wallet {
        if let Some(pos) = app.filtered_wallets.iter().position(|&i| i == selected) {
            state.select(Some(pos));
        }
    }

    frame.render_stateful_widget(list, list_layout[1], &mut state);
}

fn render_wallet_detail(frame: &mut Frame, app: &App, area: Rect) {
    if let Some(selected) = app.selected_wallet {
        if selected < app.wallet_details.len() {
            let detail = &app.wallet_details[selected];
            
            let detail_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Name
                    Constraint::Length(3), // Public Key
                    Constraint::Length(3), // Balance
                    Constraint::Length(3), // Last Transaction
                    Constraint::Min(0),    // Token Balances
                ])
                .split(area);
            
            // Wallet Name
            frame.render_widget(
                Paragraph::new(detail.name.clone())
                    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                    .block(Block::default().borders(Borders::ALL).title("Wallet Name")),
                detail_layout[0],
            );
            
            // Public Key
            let pubkey_text = match &detail.pubkey {
                Some(pubkey) => pubkey.to_string(),
                None => "Not available".to_string(),
            };
            frame.render_widget(
                Paragraph::new(pubkey_text)
                    .style(Style::default().fg(Color::Cyan))
                    .block(Block::default().borders(Borders::ALL).title("Public Key")),
                detail_layout[1],
            );
            
            // Balance
            let balance_text = match detail.balance {
                Some(balance) => format!("{:.9} SOL", balance),
                None => "Not available".to_string(),
            };
            frame.render_widget(
                Paragraph::new(balance_text)
                    .style(Style::default().fg(Color::Green))
                    .block(Block::default().borders(Borders::ALL).title("SOL Balance")),
                detail_layout[2],
            );
            
            // Last Transaction
            let tx_text = match &detail.last_transaction {
                Some(tx) => tx.clone(),
                None => "No transaction history available".to_string(),
            };
            frame.render_widget(
                Paragraph::new(tx_text)
                    .block(Block::default().borders(Borders::ALL).title("Last Transaction")),
                detail_layout[3],
            );
            
            // Token Balances
            if detail.token_balances.is_empty() {
                frame.render_widget(
                    Paragraph::new("No token balances available")
                        .alignment(Alignment::Center)
                        .block(Block::default().borders(Borders::ALL).title("Token Balances")),
                    detail_layout[4],
                );
            } else {
                let token_items: Vec<ListItem> = detail.token_balances.iter()
                    .map(|token| {
                        let line = Line::from(vec![
                            Span::styled(format!("{}: ", token.token_name), 
                                        Style::default().fg(Color::Yellow)),
                            Span::styled(format!("{:.6} ", token.amount),
                                        Style::default().fg(Color::Green)),
                            Span::styled(format!("({}...{})", 
                                               &token.mint_address[..4], 
                                               &token.mint_address[token.mint_address.len()-4..]),
                                        Style::default().fg(Color::DarkGray)),
                        ]);
                        ListItem::new(line)
                    })
                    .collect();
                
                let tokens_list = List::new(token_items)
                    .block(Block::default().borders(Borders::ALL).title("Token Balances"));
                
                frame.render_widget(tokens_list, detail_layout[4]);
            }
        } else {
            frame.render_widget(
                Paragraph::new("Wallet details not available")
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL)),
                area,
            );
        }
    } else {
        frame.render_widget(
            Paragraph::new("No wallet selected")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL)),
            area,
        );
    }
}

fn render_help(frame: &mut Frame, area: Rect) {
    let help_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Help content
        ])
        .split(area);
    
    frame.render_widget(
        Paragraph::new("svmai Wallet Manager Help")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL)),
        help_layout[0],
    );
    
    let help_text = vec![
        Line::from(vec![
            Span::styled("Navigation:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        ]),
        Line::from("  ↑/↓: Navigate wallet list"),
        Line::from("  Enter: View wallet details"),
        Line::from("  Esc/Backspace: Return to previous view"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Wallet Management:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        ]),
        Line::from("  a: Add existing wallet"),
        Line::from("  v: Create new vanity wallet with 'ai' prefix"),
        Line::from("  d: Delete selected wallet"),
        Line::from("  r: Refresh wallet list and balances"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Search and Filter:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        ]),
        Line::from("  /: Search wallets by name"),
        Line::from("  Esc: Clear search"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Operations:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        ]),
        Line::from("  b: Batch operations menu"),
        Line::from("  t: Token mixing simulation"),
        Line::from(""),
        Line::from(vec![
            Span::styled("General:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        ]),
        Line::from("  h: Show this help"),
        Line::from("  q: Quit application"),
    ];
    
    frame.render_widget(
        Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Left),
        help_layout[1],
    );
}

fn render_add_wallet(frame: &mut Frame, app: &App, area: Rect) {
    let input_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Instructions
            Constraint::Length(3), // Input field
            Constraint::Min(0),    // Extra space
        ])
        .split(area);
    
    frame.render_widget(
        Paragraph::new("Enter the path to the wallet JSON file:")
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::ALL)),
        input_layout[0],
    );
    
    frame.render_widget(
        Paragraph::new(app.input_buffer.clone())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("File Path")),
        input_layout[1],
    );
    
    frame.render_widget(
        Paragraph::new("Press Enter to confirm or Esc to cancel")
            .alignment(Alignment::Center),
        input_layout[2],
    );
}

fn render_confirm_delete(frame: &mut Frame, app: &App, area: Rect) {
    let wallet_name = if let Some(selected) = app.selected_wallet {
        if selected < app.wallets.len() {
            &app.wallets[selected]
        } else {
            "Unknown wallet"
        }
    } else {
        "Unknown wallet"
    };
    
    let confirm_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Warning
            Constraint::Length(3), // Wallet name
            Constraint::Length(3), // Confirmation options
        ])
        .split(area);
    
    frame.render_widget(
        Paragraph::new("Are you sure you want to delete this wallet?")
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Warning")),
        confirm_layout[0],
    );
    
    frame.render_widget(
        Paragraph::new(wallet_name)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Wallet to Delete")),
        confirm_layout[1],
    );
    
    let options = if app.confirm_action {
        "▶ Yes   No"
    } else {
        "Yes   ▶ No"
    };
    
    frame.render_widget(
        Paragraph::new(options)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL)),
        confirm_layout[2],
    );
}

fn render_search_wallets(frame: &mut Frame, app: &mut App, area: Rect) {
    let search_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search input
            Constraint::Min(0),    // Instructions
        ])
        .split(area);
    
    frame.render_widget(
        Paragraph::new(app.search_query.clone())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Search Query")),
        search_layout[0],
    );
    
    frame.render_widget(
        Paragraph::new("Type to search wallets by name\nPress Enter to apply search or Esc to cancel")
            .alignment(Alignment::Center),
        search_layout[1],
    );
}

fn render_batch_operations(frame: &mut Frame, _app: &App, area: Rect) {
    // This is a placeholder for batch operations UI
    // In a real implementation, this would include options for batch sending tokens
    frame.render_widget(
        Paragraph::new("Batch Operations (Coming Soon)\n\nThis feature will allow you to send tokens to multiple recipients in a single operation.")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Batch Operations")),
        area,
    );
}

fn render_create_vanity_wallet(frame: &mut Frame, app: &App, area: Rect) {
    let input_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Wallet name input
            Constraint::Length(3), // Prefix input (disabled, fixed to "ai")
            Constraint::Length(3), // Timeout input
            Constraint::Min(0),    // Instructions
        ])
        .split(area);
    
    frame.render_widget(
        Paragraph::new("Create a new wallet with a vanity address")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL)),
        input_layout[0],
    );
    
    frame.render_widget(
        Paragraph::new(app.vanity_wallet_name.clone())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Wallet Name")),
        input_layout[1],
    );
    
    frame.render_widget(
        Paragraph::new(app.vanity_config.prefix.clone())
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL).title("Address Prefix (Fixed)")),
        input_layout[2],
    );
    
    frame.render_widget(
        Paragraph::new(app.vanity_config.timeout_seconds.to_string())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Timeout (seconds)")),
        input_layout[3],
    );
    
    frame.render_widget(
        Paragraph::new("Press Enter to start generating or Esc to cancel\n\nNote: Finding a vanity address may take some time depending on the prefix.")
            .alignment(Alignment::Center),
        input_layout[4],
    );
}

fn render_vanity_progress(frame: &mut Frame, app: &App, area: Rect) {
    let progress_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Attempts
            Constraint::Length(3), // Speed
            Constraint::Length(3), // Elapsed time
            Constraint::Length(3), // Progress bar
            Constraint::Min(0),    // Instructions
        ])
        .split(area);
    
    frame.render_widget(
        Paragraph::new(format!("Generating wallet with prefix '{}'", app.vanity_config.prefix))
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL)),
        progress_layout[0],
    );
    
    // Get status information
    let (attempts, speed, elapsed) = if let Some(status) = &app.vanity_status {
        (
            status.attempts,
            status.attempts_per_second,
            status.elapsed_seconds
        )
    } else {
        (0, 0.0, 0.0)
    };
    
    frame.render_widget(
        Paragraph::new(format!("{}", attempts))
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Attempts")),
        progress_layout[1],
    );
    
    frame.render_widget(
        Paragraph::new(format!("{:.2} attempts/sec", speed))
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Speed")),
        progress_layout[2],
    );
    
    frame.render_widget(
        Paragraph::new(format!("{:.1} seconds", elapsed))
            .style(Style::default().fg(Color::Blue))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Elapsed Time")),
        progress_layout[3],
    );
    
    // Add a visual progress indicator (spinner)
    let spinner_chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let spinner_idx = (elapsed as usize / 1) % spinner_chars.len();
    let spinner = spinner_chars[spinner_idx];
    
    let progress_text = format!("{} Searching... (Press Esc to cancel)", spinner);
    
    frame.render_widget(
        Paragraph::new(progress_text)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Progress")),
        progress_layout[4],
    );
    
    frame.render_widget(
        Paragraph::new("The wallet will be saved automatically when found.\nThis may take some time depending on luck.")
            .alignment(Alignment::Center),
        progress_layout[5],
    );
}

fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let status_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70), // Status message
            Constraint::Percentage(30), // Help hint
        ])
        .split(area);
    
    // Status message
    let (status_text, status_style) = if let Some(status) = &app.status_message {
        let style = match status.status_type {
            StatusType::Info => Style::default().fg(Color::Blue),
            StatusType::Success => Style::default().fg(Color::Green),
            StatusType::Error => Style::default().fg(Color::Red),
            StatusType::Warning => Style::default().fg(Color::Yellow),
        };
        (status.message.clone(), style)
    } else {
        ("Ready".to_string(), Style::default().fg(Color::Gray))
    };
    
    frame.render_widget(
        Paragraph::new(status_text)
            .style(status_style)
            .block(Block::default().borders(Borders::ALL).title("Status")),
        status_layout[0],
    );
    
    // Help hint based on current view
    let help_hint = match app.current_view {
        View::WalletList => "h: Help | a: Add | v: Vanity | d: Delete | /: Search | Enter: Details | q: Quit",
        View::WalletDetail => "Esc: Back | r: Refresh | b: Batch Operations",
        View::Help => "Esc: Back",
        View::AddWallet => "Enter: Confirm | Esc: Cancel",
        View::ConfirmDelete => "←/→: Select | Enter: Confirm",
        View::SearchWallets => "Enter: Apply | Esc: Cancel",
        View::BatchOperations => "Esc: Back",
        View::CreateVanityWallet => "Enter: Start | Esc: Cancel",
        View::VanityProgress => "Esc: Cancel",
    };
    
    frame.render_widget(
        Paragraph::new(help_hint)
            .alignment(Alignment::Right)
            .block(Block::default().borders(Borders::ALL)),
        status_layout[1],
    );
}

// Handle key events
fn handle_key_event(app: &mut App, key_code: KeyCode) {
    match app.current_view {
        View::WalletList => handle_wallet_list_keys(app, key_code),
        View::WalletDetail => handle_wallet_detail_keys(app, key_code),
        View::Help => handle_help_keys(app, key_code),
        View::AddWallet => handle_add_wallet_keys(app, key_code),
        View::ConfirmDelete => handle_confirm_delete_keys(app, key_code),
        View::SearchWallets => handle_search_wallets_keys(app, key_code),
        View::BatchOperations => handle_batch_operations_keys(app, key_code),
        View::CreateVanityWallet => handle_create_vanity_wallet_keys(app, key_code),
        View::VanityProgress => handle_vanity_progress_keys(app, key_code),
    }
}

fn handle_wallet_list_keys(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.should_quit = true;
        },
        KeyCode::Up => {
            app.select_prev_wallet();
        },
        KeyCode::Down => {
            app.select_next_wallet();
        },
        KeyCode::Enter => {
            if app.selected_wallet.is_some() {
                app.current_view = View::WalletDetail;
            }
        },
        KeyCode::Char('h') | KeyCode::Char('H') => {
            app.current_view = View::Help;
        },
        KeyCode::Char('a') | KeyCode::Char('A') => {
            app.current_view = View::AddWallet;
            app.input_buffer.clear();
        },
        KeyCode::Char('v') | KeyCode::Char('V') => {
            app.current_view = View::CreateVanityWallet;
            app.vanity_wallet_name = "ai_wallet".to_string();
            app.vanity_config.prefix = "ai".to_string();
            app.vanity_config.timeout_seconds = 120;
        },
        KeyCode::Char('d') | KeyCode::Char('D') => {
            if app.selected_wallet.is_some() && !app.wallets.is_empty() {
                app.current_view = View::ConfirmDelete;
                app.confirm_action = false; // Default to "No"
            }
        },
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.load_wallets();
            app.set_status("Wallet list refreshed".to_string(), StatusType::Info);
        },
        KeyCode::Char('/') => {
            app.current_view = View::SearchWallets;
            app.search_query.clear();
        },
        KeyCode::Char('b') | KeyCode::Char('B') => {
            app.current_view = View::BatchOperations;
        },
        _ => {}
    }
}

fn handle_wallet_detail_keys(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Esc | KeyCode::Backspace => {
            app.current_view = View::WalletList;
        },
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.load_wallets();
            app.set_status("Wallet details refreshed".to_string(), StatusType::Info);
        },
        KeyCode::Char('b') | KeyCode::Char('B') => {
            app.current_view = View::BatchOperations;
        },
        _ => {}
    }
}

fn handle_help_keys(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Esc | KeyCode::Backspace | KeyCode::Char('q') => {
            app.current_view = View::WalletList;
        },
        _ => {}
    }
}

fn handle_add_wallet_keys(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Esc => {
            app.current_view = View::WalletList;
            app.input_buffer.clear();
        },
        KeyCode::Enter => {
            let input_buffer_clone = app.input_buffer.clone();
            app.add_wallet(input_buffer_clone);
        },
        KeyCode::Backspace => {
            app.input_buffer.pop();
        },
        KeyCode::Char(c) => {
            app.input_buffer.push(c);
        },
        _ => {}
    }
}

fn handle_confirm_delete_keys(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Esc => {
            app.current_view = View::WalletList;
        },
        KeyCode::Left => {
            app.confirm_action = true; // Yes
        },
        KeyCode::Right => {
            app.confirm_action = false; // No
        },
        KeyCode::Enter => {
            if app.confirm_action {
                app.remove_selected_wallet();
            } else {
                app.current_view = View::WalletList;
            }
        },
        _ => {}
    }
}

fn handle_search_wallets_keys(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Esc => {
            app.search_query.clear();
            app.update_filtered_wallets();
            app.current_view = View::WalletList;
        },
        KeyCode::Enter => {
            app.update_filtered_wallets();
            app.current_view = View::WalletList;
        },
        KeyCode::Backspace => {
            app.search_query.pop();
        },
        KeyCode::Char(c) => {
            app.search_query.push(c);
        },
        _ => {}
    }
}

fn handle_batch_operations_keys(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Esc | KeyCode::Backspace => {
            app.current_view = View::WalletList;
        },
        _ => {}
    }
}

fn handle_create_vanity_wallet_keys(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Esc => {
            app.current_view = View::WalletList;
        },
        KeyCode::Enter => {
            app.start_vanity_wallet_creation();
        },
        KeyCode::Backspace => {
            app.vanity_wallet_name.pop();
        },
        KeyCode::Char(c) => {
            // Only allow editing the wallet name, prefix is fixed to "ai"
            app.vanity_wallet_name.push(c);
        },
        _ => {}
    }
}

fn handle_vanity_progress_keys(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Esc => {
            app.cancel_vanity_generation();
        },
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    // TUI tests are often integration-style or snapshot tests, which can be complex.
    // For now, we will skip detailed TUI rendering tests.
    // Basic state logic tests can be added here.
    use super::*;

    #[test]
    fn test_app_new() {
        let app = App::new();
        assert!(app.wallets.is_empty());
        assert_eq!(app.selected_wallet, None);
        assert!(!app.should_quit);
        assert!(matches!(app.current_view, View::WalletList));
    }
    
    #[test]
    fn test_update_filtered_wallets() {
        let mut app = App::new();
        app.wallets = vec![
            "wallet1".to_string(),
            "wallet2".to_string(),
            "test_wallet".to_string(),
        ];
        
        // Test with empty search query (should include all wallets)
        app.search_query = "".to_string();
        app.update_filtered_wallets();
        assert_eq!(app.filtered_wallets, vec![0, 1, 2]);
        
        // Test with matching search query
        app.search_query = "test".to_string();
        app.update_filtered_wallets();
        assert_eq!(app.filtered_wallets, vec![2]);
        
        // Test with non-matching search query
        app.search_query = "nonexistent".to_string();
        app.update_filtered_wallets();
        assert!(app.filtered_wallets.is_empty());
    }
    
    #[test]
    fn test_wallet_navigation() {
        let mut app = App::new();
        app.wallets = vec![
            "wallet1".to_string(),
            "wallet2".to_string(),
            "wallet3".to_string(),
        ];
        app.filtered_wallets = vec![0, 1, 2];
        
        // Test initial selection
        assert_eq!(app.selected_wallet, None);
        
        // Test selecting next wallet
        app.select_next_wallet();
        assert_eq!(app.selected_wallet, Some(0));
        
        app.select_next_wallet();
        assert_eq!(app.selected_wallet, Some(1));
        
        app.select_next_wallet();
        assert_eq!(app.selected_wallet, Some(2));
        
        // Test boundary (should not go beyond last wallet)
        app.select_next_wallet();
        assert_eq!(app.selected_wallet, Some(2));
        
        // Test selecting previous wallet
        app.select_prev_wallet();
        assert_eq!(app.selected_wallet, Some(1));
        
        app.select_prev_wallet();
        assert_eq!(app.selected_wallet, Some(0));
        
        // Test boundary (should not go before first wallet)
        app.select_prev_wallet();
        assert_eq!(app.selected_wallet, Some(0));
    }

    // More tests would require mocking wallet_manager or having a test setup for it.
}
