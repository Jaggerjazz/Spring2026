mod bank_account;
use bank_account::BankAccount;

fn main() {
    let mut account = BankAccount::new(1000.0);
    println!("Initial: ${:.2}", account.balance());

    account.deposit(500.0);
    println!("After Deposit: ${:.2}", account.balance());

    account.withdraw(200.0);
    println!("After Withdrawal: ${:.2}", account.balance());

    account.deposit(-100.0);
    println!("After Negative Deposit: ${:.2}", account.balance());

    account.withdraw(2000.0);
    println!("After Over-withdrawal: ${:.2}", account.balance());
}