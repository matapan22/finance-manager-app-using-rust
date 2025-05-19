# Welcome to Finance Management App

The **Finance Manager App** is a user-friendly desktop application designed to help individuals track their income, expenses, and budgets efficiently. Built using Rust and GTK4, the app provides an intuitive graphical interface for users to manage transactions, view insights, and analyze financial data in real-time.

With this tool, you can:

-   **Transaction Management:**    Add income and expense transactions easily, Categorize transactions (e.g., Food, Rent, Transport), View and edit past transactions.
-   **Budget Tracking:** Set a monthly budget to manage expenses, Monitor total income vs. total expenses, Visualize financial trends and insights.
-   **Database Integration (SQLite):** Store transaction history securely in a SQLite database, Retrieve and display the transactions in a table, Prevent data loss with persistent storage.
-   **Interactive UI with GTK4:** Dropdown selection for category selection Toggle buttons to switch between income & expense, Popup windows for alerts and messages, dynamic tables to display transaction records.
# Table of Contents

- [Prerequisites](#prerequisites) 
- [Installation and Setup](#installation_and_setup) 
- [Steps_to_Generate_Executable](#steps_to_generate_executable)
- [ Project Structure](#project_structure)  
-  [Dependencies](#dependencies) 
-  [Screenshots](#screenshots) 
-  [Future_Enhancements](#future_enhancements)
-  [Contributing](#contributing)
-  [License](#license)
- [Authors](#authors)
- [License](#license)

# Prerequisites
Before running the project, ensure you have the following installed:
### **Install Dependencies:**
```bash
# Install Rust & Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install GTK4 and Required Packages (Ubuntu/Debian)
sudo apt update
sudo apt install -y libgtk-4-dev glib-compile-resources sqlite3 pkg-config
# On Fedora/RHEL
sudo dnf install gtk4-devel sqlite
# On Arch Linux
sudo pacman -S gtk4 sqlite
# Install Required Rust Crates
cargo install cargo-edit
```
# Installation_and_Setup
1. Clone the Repository
```bash
git clone https://gitlab.rlp.net/mathew.siby/finance-manager-app-using-rust
cd finance-manager-app
```
2. Compile UI Resources
```bash
glib-compile-resources src/resources/resources.gresource.xml --target=resources.gresource
```
3. Run the Application
```bash
cargo run
```
# Steps_to_Generate_Executable
Follow these steps to build and run the application as an executable:
1. Build the Project
```bash
cargo build --release
```
The compiled binary will be found in:
```bash
target/release/my_finance_app
```
2. Run the Executable
Navigate to the `target/release` directory and run the app:
```bash
./target/release/my_finance_app
```
# Project_Structure
```bash
finance-manager-app/
│── src/
│   ├── main.rs                		# Main application entry point
│   ├── transactionwindow.rs     	# Handles transactions window
│   ├── viewtransactions.rs      	# Displays all transactions
│   ├── finance   				 	# Database and finance logic
│   │   ├── expenses.rs  			# Expense logics
│   │   ├── income.rs 				# Income logics
│   │   ├── mod.rs 					# module for transaction processing
│   │   ├── transaction.rs 			# transaction logic
│   ├── resources/                  # UI Files
│   │   ├── window.ui               # Main Window UI (GTK4 XML)
│   │   ├── transactionwindow.ui 	# Add Transactions UI
│   │   ├── viewtransactions.ui  	# View Transactions UI
│   │   ├── resources.gresource.xml # Resource definition
│── Cargo.toml                   	# Rust dependencies
│── README.md                    	# Documentation
│── Cargo.lock	                 	# Exact information about your dependencies
│── Finance_app.db                	# SQLite Database 
│── build.rs	                	# Script to compile execute
│── target		                	# Contains the build artefacts
│── LICENSE		                	# License
│── CONTRIBUTING.md                	# Contribution
```
# Dependencies
-   **[GTK4](https://gtk-rs.org/)** – For GUI development
-   **[Rusqlite](https://github.com/rusqlite/rusqlite)** – SQLite database for storing transactions
-   **Once Cell** – For managing global state
-   **Chrono** – For date & time handling
# Screenshots
Screenshots available at https://gitlab.rlp.net/mathew.siby/finance-manager-app-using-rust/-/tree/main/my_finance_app/Screenshots?ref_type=heads
# Future_Enhancements
- Add Pie Chart for Expense Breakdown  
- Export Transaction History as CSV/PDF  
- User defined category
- Insights window 

# Contributing
Contributions are welcome! To contribute:

1.  Fork the repository.
2.  Create a new branch (`feature-new`).
3.  Commit your changes (`git commit -m "Add new feature"`).
4.  Push and open a Pull Request.
# License
This project is licensed under the **MIT License**.
# Authors
- **Albayrak Salih** – Backend Development - salih.albayrak@th-bingen.de
-   **Siby Mathew** – UI Development - email: mathew.siby@th-bingen.de
- **Andaythu Raveendran Oormila** – Database - email: oormila.andaythu@th-bingen.de

For any issues, feel free to open a GitHub issue or contact us.
## Acknowledgments
- Thanks to [Rust Programming Language](https://www.rust-lang.org/) for providing powerful tools.
- Inspired by [GTK4](https://www.gtk.org/) for UI development.
