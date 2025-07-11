# TTAWin Project Aliases for Development

# --- NPM/Yarn/Tauri ---
alias tauri-dev='npm run tauri dev --prefix winapp'
alias tauri-build='npm run tauri build --prefix winapp'
alias tauri-info='npm run tauri info --prefix winapp'
alias tauri-clean='npm run tauri clean --prefix winapp'

# --- Cargo (Rust) ---
alias cbuild='cargo build --workspace'
alias ctest='cargo test --workspace'
alias cclippy='cargo clippy --workspace --all-targets --all-features'
alias cfmt='cargo fmt --all'
alias cdoc='cargo doc --workspace --open'
alias crun='cargo run --workspace'

# --- Project Navigation ---
alias goto-root='cd ~/Documents/GitHub/TTAWin'
alias goto-winapp='cd ~/Documents/GitHub/TTAWin/winapp'
alias goto-learning='cd ~/Documents/GitHub/TTAWin/packages/learning'
alias goto-payments='cd ~/Documents/GitHub/TTAWin/packages/payments'

# --- NPM/Yarn in winapp ---
alias nwin='cd winapp && npm'
alias ninstall='cd winapp && npm install'
alias nbuild='cd winapp && npm run build'
alias ndev='cd winapp && npm run dev'

# --- Misc ---
alias reload-zsh='source ~/.zshrc'

# Usage:
#   source .zshrc
#   Then use: tauri-dev, cbuild, ctest, goto-winapp, etc.
