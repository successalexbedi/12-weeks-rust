#[derive(Debug)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed { amount: f64 },
    Failed { reason: String },
}

pub fn create_pending() -> PaymentStatus {
    PaymentStatus::Pending
}

pub fn create_completed(amount: f64) -> PaymentStatus {
    PaymentStatus::Completed { amount }
}

pub fn create_failed(reason: String) -> PaymentStatus {
    PaymentStatus::Failed { reason }
}

pub fn display_status(status: &PaymentStatus) {
    match status {
        PaymentStatus::Pending => println!("Payment is pending"),
        PaymentStatus::Processing => println!("Payment is processing"),
        PaymentStatus::Completed { amount } => println!("Payment completed: ${amount}"),
        PaymentStatus::Failed { reason } => println!("Payment failed: {reason}"),
    }
}

pub fn is_final(status: &PaymentStatus) -> bool {
    match status {
        PaymentStatus::Completed { .. } | PaymentStatus::Failed { .. } => true,
        _ => false,
    }
}

pub fn get_amount(status: &PaymentStatus) -> Option<f64> {
    match status {
        PaymentStatus::Completed { amount } => Some(*amount),
        _ => None,
    }
}

fn main() {
    // 1. Create 5 payment statuses (All variants)
    let mut history = Vec::new();
    
    history.push(create_pending());
    history.push(PaymentStatus::Processing);
    history.push(create_completed(99.99));
    history.push(create_completed(15.50));
    history.push(create_failed(String::from("Insufficient funds")));

    println!("--- PAYMENT PROCESSING LOG ---");

    // 2. Loop through and process each status
    for (i, status) in history.iter().enumerate() {
        println!("\n[Transaction #{}]", i + 1);
        
        // 3. Display the info
        display_status(status);

        // 4. Check if final
        if is_final(status) {
            println!("Status: ✅ DONE");
        } else {
            println!("Status: ⏳ IN PROGRESS");
        }

        // 5. Try to get amount using Option matching
        match get_amount(status) {
            Some(amt) => println!("Amount: ${}", amt),
            None => println!("Amount: N/A"),
        }
    }

    println!("\n--- LOG COMPLETE ---");
}
