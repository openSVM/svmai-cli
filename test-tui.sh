#!/bin/bash
# TUI Testing Script for svmai-cli
# This script demonstrates how to test and record TUI sessions

set -e

echo "==================================="
echo "svmai-cli TUI Testing Script"
echo "==================================="
echo ""

# Check if asciinema is installed
if ! command -v asciinema &> /dev/null; then
    echo "ERROR: asciinema is not installed"
    echo "Install it with: sudo apt-get install asciinema"
    exit 1
fi

# Build the project
echo "Step 1: Building svmai-cli..."
cargo build --release
echo "✓ Build completed successfully"
echo ""

# Set up test environment
echo "Step 2: Setting up test environment..."
export SVMAI_TEST_SERVICE_NAME="svmai_test_$(date +%s)"
mkdir -p /tmp/svmai-test-wallets
echo "✓ Test environment ready"
echo ""

# Create test wallet files for demonstration
echo "Step 3: Creating test wallet files..."
# Note: These would be actual Solana wallet JSON files in a real test
# For demo purposes, we're documenting the structure
cat > /tmp/svmai-test-wallets/test_wallet_1.json << 'EOF'
[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64]
EOF
echo "✓ Test wallet files created"
echo ""

# Test recording configurations
RECORDINGS_DIR="/tmp/svmai-recordings"
mkdir -p "$RECORDINGS_DIR"

echo "==================================="
echo "Recording Sessions"
echo "==================================="
echo ""
echo "The following sessions should be recorded:"
echo ""

echo "1. FIRST-TIME LAUNCH AND SETUP"
echo "   File: ${RECORDINGS_DIR}/01-first-launch.cast"
echo "   Steps:"
echo "   - Launch svmai"
echo "   - Observe initial empty wallet list"
echo "   - Press 'H' to view help screen"
echo "   - Press 'Esc' to return to main view"
echo ""

echo "2. ADDING A WALLET"
echo "   File: ${RECORDINGS_DIR}/02-add-wallet.cast"
echo "   Steps:"
echo "   - Press 'A' to add wallet"
echo "   - Enter path: /tmp/svmai-test-wallets/test_wallet_1.json"
echo "   - Observe success message"
echo "   - View wallet in list"
echo ""

echo "3. WALLET NAVIGATION"
echo "   File: ${RECORDINGS_DIR}/03-navigation.cast"
echo "   Steps:"
echo "   - Use arrow keys to navigate wallet list"
echo "   - Press 'Enter' to view wallet details"
echo "   - Observe public key, balance, and token information"
echo "   - Press 'Esc' to return to list"
echo ""

echo "4. SEARCH FUNCTIONALITY"
echo "   File: ${RECORDINGS_DIR}/04-search.cast"
echo "   Steps:"
echo "   - Press '/' to activate search"
echo "   - Type search query"
echo "   - Press 'Enter' to filter results"
echo "   - Press 'Esc' to clear search"
echo ""

echo "5. VANITY WALLET CREATION"
echo "   File: ${RECORDINGS_DIR}/05-vanity-wallet.cast"
echo "   Steps:"
echo "   - Press 'V' to create vanity wallet"
echo "   - Enter wallet name (e.g., 'my_ai_wallet')"
echo "   - Observe real-time progress:"
echo "     * Attempts counter"
echo "     * Speed (attempts/sec)"
echo "     * Elapsed time"
echo "     * Progress indicator"
echo "   - Wait for completion or press 'Esc' to cancel"
echo "   - View newly created wallet in list"
echo ""

echo "6. REFRESH WALLETS"
echo "   File: ${RECORDINGS_DIR}/06-refresh.cast"
echo "   Steps:"
echo "   - Press 'R' to refresh wallet list"
echo "   - Observe status message"
echo "   - Note updated balances (if network available)"
echo ""

echo "7. DELETE WALLET"
echo "   File: ${RECORDINGS_DIR}/07-delete-wallet.cast"
echo "   Steps:"
echo "   - Select a wallet"
echo "   - Press 'D' to delete"
echo "   - Observe confirmation dialog"
echo "   - Confirm or cancel deletion"
echo ""

echo "8. BATCH OPERATIONS"
echo "   File: ${RECORDINGS_DIR}/08-batch-operations.cast"
echo "   Steps:"
echo "   - Press 'B' to access batch operations menu"
echo "   - Navigate through available options"
echo "   - Return to main view"
echo ""

echo "9. KEYBOARD SHORTCUTS DEMO"
echo "   File: ${RECORDINGS_DIR}/09-shortcuts-demo.cast"
echo "   Steps:"
echo "   - Demonstrate all keyboard shortcuts in sequence"
echo "   - Show status bar updates"
echo "   - Show context-sensitive help"
echo ""

echo "10. ERROR HANDLING"
echo "   File: ${RECORDINGS_DIR}/10-error-handling.cast"
echo "   Steps:"
echo "   - Try to add invalid wallet file"
echo "   - Observe error message"
echo "   - Try to access non-existent wallet"
echo "   - Show graceful error recovery"
echo ""

echo ""
echo "==================================="
echo "Recording Commands"
echo "==================================="
echo ""
echo "To record a session, use:"
echo "  asciinema rec ${RECORDINGS_DIR}/session-name.cast"
echo ""
echo "To record with title and metadata:"
echo "  asciinema rec -t \"Session Title\" ${RECORDINGS_DIR}/session-name.cast"
echo ""
echo "To play back a recording:"
echo "  asciinema play ${RECORDINGS_DIR}/session-name.cast"
echo ""
echo "To convert to GIF (requires agg or other tools):"
echo "  # Install agg: cargo install agg"
echo "  agg ${RECORDINGS_DIR}/session-name.cast ${RECORDINGS_DIR}/session-name.gif"
echo ""
echo "To upload to asciinema.org:"
echo "  asciinema upload ${RECORDINGS_DIR}/session-name.cast"
echo ""

echo "==================================="
echo "Manual Testing Checklist"
echo "==================================="
echo ""
cat << 'CHECKLIST'
[ ] TUI launches without errors
[ ] Initial screen shows empty wallet list
[ ] Help screen displays all shortcuts
[ ] Add wallet accepts valid wallet files
[ ] Add wallet rejects invalid files with clear error
[ ] Wallet list shows correct information
[ ] Navigation with arrow keys works smoothly
[ ] Enter key opens wallet detail view
[ ] Wallet detail shows complete information
[ ] Search activates with '/' key
[ ] Search filters wallets correctly
[ ] Search clears with Esc key
[ ] Vanity wallet creation starts correctly
[ ] Vanity progress updates in real-time
[ ] Vanity generation can be cancelled
[ ] Vanity wallet is saved when found
[ ] Delete shows confirmation dialog
[ ] Delete removes wallet correctly
[ ] Refresh updates wallet list
[ ] Status bar shows appropriate messages
[ ] Status messages have correct colors
[ ] All keyboard shortcuts work as documented
[ ] UI adapts to terminal resize
[ ] No visual artifacts or rendering issues
[ ] Application quits cleanly with 'Q'
[ ] Batch operations menu is accessible
[ ] Network operations handle errors gracefully
[ ] Keychain integration works (on supported systems)
[ ] Private keys are properly encrypted
[ ] No sensitive data in status messages
[ ] Performance is responsive with many wallets
CHECKLIST

echo ""
echo "==================================="
echo "Testing on Different Platforms"
echo "==================================="
echo ""
echo "macOS:"
echo "  - Test with macOS Keychain"
echo "  - Verify Keychain Access prompts"
echo "  - Test in Terminal.app and iTerm2"
echo ""
echo "Linux:"
echo "  - Test with GNOME Keyring"
echo "  - Test with KeePassXC"
echo "  - Test in various terminal emulators (gnome-terminal, konsole, xterm)"
echo ""
echo "Windows:"
echo "  - Test with Windows Credential Manager"
echo "  - Test in Windows Terminal"
echo "  - Test in Git Bash"
echo ""

echo "==================================="
echo "Performance Testing"
echo "==================================="
echo ""
echo "Test with varying number of wallets:"
echo "  - 1 wallet: Baseline"
echo "  - 10 wallets: Normal usage"
echo "  - 50 wallets: Heavy usage"
echo "  - 100+ wallets: Stress test"
echo ""
echo "Measure:"
echo "  - UI responsiveness"
echo "  - Memory usage"
echo "  - Load time"
echo "  - Search performance"
echo ""

echo "==================================="
echo "Test Complete"
echo "==================================="
echo ""
echo "For actual TUI testing, run:"
echo "  ./target/release/svmai"
echo ""
echo "Note: Ensure you have a working keychain service"
echo "and appropriate permissions to access it."
echo ""
