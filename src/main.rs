use std::io::{self, Write};

struct Wallet {
    address: String,
    balance: f64,
}

enum MenuOption {
    Wallet,
    Balance,
    Send,
    Exit,
    Invalid,
}

impl MenuOption {
    fn from_input(input: &str) -> Self {
        match input.trim() {
            "1" => MenuOption::Wallet,
            "2" => MenuOption::Balance,
            "3" => MenuOption::Send,
            "4" => MenuOption::Exit,
            _ => MenuOption::Invalid,
        }
    }
}

fn main() {
    let wallet = Wallet {
        address: String::from("DemoSolanaWallet123"),
        balance: 1.50,
    };

    println!("================================");
    println!("      SOLANA DEVELOPER CLI");
    println!("================================");

    loop {
        println!();
        println!("1. Show wallet");
        println!("2. Show balance");
        println!("3. Send SOL");
        println!("4. Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match MenuOption::from_input(&input) {
            MenuOption::Wallet => {
                println!("Wallet address: {}", wallet.address);
            }

            MenuOption::Balance => {
                println!("Balance: {:.2} SOL", wallet.balance);
            }

            MenuOption::Send => {
                send_sol(&wallet);
            }

            MenuOption::Exit => {
                println!("Goodbye!");
                break;
            }

            MenuOption::Invalid => {
                println!("Invalid option. Try again.");
            }
        }
    }
}

fn send_sol(wallet: &Wallet) {
    println!("Current balance: {:.2} SOL", wallet.balance);
    println!("Transaction simulation: 0.1 SOL");
    println!("Transaction status: SUCCESS");
}