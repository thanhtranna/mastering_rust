enum PaymentMode {
    Debit,
    Credit,
    Paypal,
}

// Bunch of dummy payment handlers
fn pay_by_credit(atm: u64) {
    println!("Processing credit payment of {}", atm);
}

fn pay_by_debit(atm: u64) {
    println!("Processing debit payment of {}", atm);
}

fn paypal_redirect(atm: u64) {
    println!("Processing to paypal for amount: {}", atm);
}

impl PaymentMode {
    fn pay(&self, amount: u64) {
        match self {
            PaymentMode::Debit => pay_by_debit(amount),
            PaymentMode::Credit => pay_by_credit(amount),
            PaymentMode::Paypal => paypal_redirect(amount),
        }
    }
}

fn get_saved_payment_mode() -> PaymentMode {
    PaymentMode::Debit
}

fn main() {
    let payment_mode = get_saved_payment_mode();

    payment_mode.pay(512);
}
