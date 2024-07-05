pub struct Wallet {
    pub balance: u64,
    // Other fields and methods as needed
}

impl Wallet {
    pub fn new() -> Self {
        // Initialize your wallet
        Self { balance: 1000 }
    }

    pub fn send_tokens(&mut self, recipient: &str, amount: u64) {
        // Logic to send tokens
        if self.balance < amount {
            panic!("Insufficient balance");
        }
        self.balance -= amount;
        println!("Sent {} tokens to {}", amount, recipient);
    }

    pub fn display_balance(&self) {
        println!("Current balance: {}", self.balance);
    }
}


