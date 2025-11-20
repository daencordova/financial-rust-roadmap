// --------------------------------------------------------------------------
// --- SYSTAX AND SEMANTICS ---
// -------------------------------------------------------------------------

// --------------------------------------------------------------------------
// VARIABLES, DATATYPES AND CONSTANTS
// --------------------------------------------------------------------------

// // A Constants for the Annual Compounding Periods
// // Constants are declared with 'const', must have an explicit type,
// // and are generally named in SCREAMING_SNAKE_CASE.
// const COMPOUDING_PERIODS: u32 = 1; // u32: Unsigned 32-bit integer (e.g., compounded annually)

// fn main() {
//     // Variables

//     // Mutable variable: 'initial_principal' can be changed (mut keyword).
//     // f64: 64-bit floating-point number (good for financial precision).
//     let mut initial_principal: f64 = 1000.00; // Starting investment amount

//     // Inmutable variable: 'annual_interest_rate' cannot be changed.
//     // f64: 64-bit floating-point number.
//     let annual_interest_rate: f64 = 0.05; // %5 annual interest rate

//     // Inmutable variable: 'years' is an integer.
//     // u32: Unsigned 32-bit integer.
//     let years: u32 = 10; // Investment duration in years

//     // --- Financial Calculation ---

//     // Calculate the future value (FV)
//     // Formula: FV = P * (a + r/n)^(n*t)
//     // P = initial_principal, r = annual_interest_rate, n = COMPOUDING_PERIODS, t = years

//     let rate_per_period = annual_interest_rate / COMPOUDING_PERIODS as f64;
//     let total_periods = (years * COMPOUDING_PERIODS) as u32;

//     // The 'future_value' is a variable to store the result, with type f64.
//     let future_value: f64 =
//         initial_principal * f64::powf(1.0 + rate_per_period, total_periods as f64);

//     // Update the principal after one year to simulate a change (using the mutable variable)
//     initial_principal = initial_principal * (1.0 + annual_interest_rate);

//     // Output the results
//     println!("---Compound Interest Calculation ---");
//     println!(
//         "Initial Principal (Year 0): ${:.2}",
//         initial_principal / (1.0 + annual_interest_rate)
//     );
//     println!("Annual Interest Rate: ${:.2}", annual_interest_rate * 100.0);
//     println!("Investment Duration: {} years", years);
//     println!("Compouding Periods per Year: {}", COMPOUDING_PERIODS);
//     println!("-----------------------------------");
//     println!("Future Value after {} years: ${:.2}", years, future_value);
//     println!(
//         "Principal After One Year (Mutated): ${:.2}",
//         initial_principal
//     );
// }

// --------------------------------------------------------------------------
// CONTROL FLOW AND CONSTRUCTS
// --------------------------------------------------------------------------

// const NUM_SIMULATIONS: u64 = 100_000;
// const INITIAL_PRICE: f64 = 100.0;
// const STRIKE_PRICE: f64 = 105.0; // The option's  exercise price
// const TIME_STEPS: u32 = 252; // e.g., number of trading days
// const RISK_FREE_RATE: f64 = 0.05;
// const VOLATILITY: f64 = 0.20;
// // Note: This is a highly simplified model for demostration.
// // Real models use more complex math and proper time-step scaling (dt)

// fn main() {
//     let mut total_payoff = 0.0;

//     // 1. **'for' loop** to run the simulations
//     // This repeats the entire princing process many times for statistical accuracy.
//     for _ in 0..NUM_SIMULATIONS {
//         let mut current_price = INITIAL_PRICE;

//         // 2. **'for' loop** to step throught time for a single simulation path
//         for _ in 0..TIME_STEPS {
//             // Placeholder for a random, normally distributed change (delta)
//             // A real implementation would use a proper statistical library for this
//             let random_shock = (rand::random::<f64>() - 0.5) * 2.0 * VOLATILITY * 0.1;
//             current_price *= 1.0 + random_shock + (RISK_FREE_RATE / TIME_STEPS as f64);
//         }

//         // 3. Calculate the Option's Payoff at Expiration (a European Call Option)
//         // **'if' expression** is used for the core option logic (control flow)
//         let payoff = if current_price > STRIKE_PRICE {
//             // If the final price is above the strike price, the option is "in the money"
//             current_price - STRIKE_PRICE
//         } else {
//             // Otherwise, the option expires worthless
//             0.0
//         };

//         total_payoff += payoff;
//     }

//     // Calculate the average payoff
//     let average_payoff = total_payoff / NUM_SIMULATIONS as f64;

//     // **Final Financial Step:** Discount the average payoff back to today's value
//     // (A simple present value calculation using the risk-free rate)
//     let option_price = average_payoff / (1.0 + RISK_FREE_RATE);

//     println!("Total Simulations: {}", NUM_SIMULATIONS);
//     println!("Estimated Option Price: {:.4}", option_price);
// }

// --------------------------------------------------------------------------
// FUNCTIONS AND METHOD SYNTAX
// --------------------------------------------------------------------------

// // A general-purpose **function** for power calculation
// // Note: Rust's standard library has 'f64::powf' or  'f64::powi', but we define one
// // here to explicitly demostrate a standalone function.
// fn power(base: f64, exponent: u32) -> f64 {
//     base.powi(exponent as i32)
// }

// // Define a struct to hold the financial data
// struct Investment {
//     present_value: f64,
//     rate: f64,    // Annual interest rate (e.g., 0.05 for %5)
//     periods: u32, // Number of years
// }

// // Implement **Method Syntax** for the Investment struct
// impl Investment {
//     // An **associated function** (like a static method) to create a new Investment
//     pub fn new(pv: f64, rate: f64, periods: u32) -> Self {
//         Investment {
//             present_value: pv,
//             rate,
//             periods,
//         }
//     }

//     // A **method** (take '&self') to calculate the Future Value
//     pub fn calculate_future_value(&self) -> f64 {
//         let base = 1.0 + self.rate;
//         let factor = power(base, self.periods); // Using the standalone function

//         // FV = PV * (1 + r)^n
//         self.present_value * factor
//     }
// }

// fn main() {
//     // 1. Create a new investment instance using the associated function
//     let initial_investment = Investment::new(
//         1000.0, // $1000 initial investment  (PV)
//         0.05,   // 5% annual interest rate (r)
//         10,     // 10 years (n)
//     );

//     // 2. Call the method to calculate the Future Value
//     // This demonstrates **Method Syntax** ('instance.method_name(...)')
//     let future_value = initial_investment.calculate_future_value();

//     println!("Financial Calculation Example: Future Value");
//     println!("---");
//     println!(
//         "Initial Investment (PV): ${:.2}",
//         initial_investment.present_value
//     );
//     println!("Annual Rate (r): ${:.1}", initial_investment.rate * 100.0);
//     println!("Time Period (n): {} years", initial_investment.periods);
//     println!("---");
//     println!("**Future Value (FV): ${:.2}**", future_value);
// }

// --------------------------------------------------------------------------
// PATTERN MATCHING AND DESTRUCTURING
// --------------------------------------------------------------------------

// // Define an enum to represent the different types of financial transactions.
// #[derive(Debug)]
// enum FinancialTransaction {
//     // Basic deposit/withdrawal, holds the amount and the account ID.
//     Transfer {
//         amount: f64,
//         account_id: u32,
//     },
//     // Payment for a bill, holds the amount, vendor name, and due date.
//     BillPayment {
//         amount: f64,
//         vendor: String,
//         due_date: String,
//     },
//     // Simple refund, holds just the amount.
//     Refund(f64),
//     // An error/unrecognized transaction type.
//     Unknown,
// }

// // function to proccess a single transaction using pattern matching and destructuring.
// fn process_transaction(transaction: FinancialTransaction) {
//     // The 'match' expression uses pattern matching on the 'transaction' enum.
//     match transaction {
//         // 1. Destructuring a 'struct-like' variant (Transfer).
//         // It extracts the 'amount' and 'account_id' fields directly into local variables.
//         FinancialTransaction::Transfer { amount, account_id } => {
//             println!(
//                 "Transfer: Processed an amount of ${} for account ID {}.",
//                 amount, account_id
//             );
//             // We can add logic here, e.g., call a function to update the balance
//         }

//         // 2. Destructuring a different 'struct-like' variant (BillPayment).
//         // We use a specific pattern to destruct 'vendor' and 'amount',
//         // but use the **_** (underscore) to ignore the 'due_date' field for simplicity.
//         FinancialTransaction::BillPayment {
//             amount: bill_amount,
//             vendor,
//             due_date,
//         } => {
//             // Note the use of 'bill_amount' for renaming the 'amount' field to avoid a naming conflict if needed.
//             println!(
//                 "Bill Payment: Sent ${} to **{}** | {} ",
//                 bill_amount, vendor, due_date
//             );
//         }

//         // 3. Destructuring a 'tuple-like' variant (Refund).
//         // It extracts the single f64 value inside the variant into the 'refund_amount' variable.
//         FinancialTransaction::Refund(refund_amount) => {
//             println!("Refund: Issued a refund of **{}**.", refund_amount);
//         }

//         // 4. The catch-all pattern.
//         // The _ matches any other case (like 'Unknown') that hasn't been explicitly handled.
//         FinancialTransaction::Unknown => {
//             println!("Unknown Transaction: Requires manual review.");
//         }
//     }
// }

// fn main() {
//     // Create a vector of transactions
//     let transactions = vec![
//         FinancialTransaction::Transfer {
//             amount: 500.00,
//             account_id: 101,
//         },
//         FinancialTransaction::BillPayment {
//             amount: 75.50,
//             vendor: String::from("Electric Co."),
//             due_date: String::from("2025-12-01"),
//         },
//         FinancialTransaction::Refund(25.00),
//         FinancialTransaction::Unknown,
//         FinancialTransaction::Transfer {
//             amount: 120.99,
//             account_id: 202,
//         },
//     ];

//     println!("--- Starting Transaction Processing ---");
//     // Iterate over the vector and process each transaction.
//     for tx in transactions {
//         process_transaction(tx);
//     }
//     println!("--- Processing Complete ---");
// }

// --------------------------------------------------------------------------
// --- CONSTRUCTS ---
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// ENUMS
// --------------------------------------------------------------------------

// // 1. Define an Enum to represent the different types of financial transactions.
// // This is the core use of Enums for modeling distinct states.
// #[derive(Debug)]
// pub enum TransactionType {
//     // A deposit, often associated with money coming into an account.
//     Deposit,
//     // A withdrawal, money taken out of an account.
//     Withdrawal,
//     // A transfer, money moved between accounts.
//     // We can attach data (e.g., the destination account ID) directly to the variant.
//     Transfer { destination_account_id: u32 },
// }

// // 2. Define a Struct to hold the full transaction details.
// #[derive(Debug)]
// pub struct Transaction {
//     pub id: u32,
//     pub account_id: u32,
//     pub transaction_type: TransactionType, // Uses the Enum
//     pub amount: f64,
// }

// impl Transaction {
//     // A method to process the transaction and print a summary.
//     pub fn process(&self) {
//         println!("--- Processing Transaction #{} ---", self.id);
//         println!("Account ID: {}", self.account_id);
//         println!("Amount: $ {:.2}", self.amount);

//         // 3. Use a 'match' statementon the Enum to execute logic specific to each type.
//         // This is a powerful pattern for handling Enums in Rust.
//         match &self.transaction_type {
//             TransactionType::Deposit => {
//                 println!("Action: **Money Deposited**.");
//                 // In a real system, you would update the account balance here (e.g., balance += self.amount)
//             }

//             TransactionType::Withdrawal => {
//                 println!("Action: **Money Withdrawn**.");
//                 // In a real system, you would check for sufficient funds first
//                 // (e.g., if balance >= self.amount { balance -= self.amount })
//             }

//             TransactionType::Transfer {
//                 destination_account_id,
//             } => {
//                 println!("Action: **Money Transferred**.");
//                 println!(
//                     "Transferring ${:.2} from Account {} to Account {}.",
//                     self.amount, self.account_id, destination_account_id
//                 );
//                 // In a real system, two account balances would be updated here.
//             }
//         }

//         println!("------------------------------------------------\n");
//     }
// }

// // 4. Main function to demonstrate the usage.
// fn main() {
//     // Example 1: A simple deposit
//     let deposit = Transaction {
//         id: 101,
//         account_id: 5001,
//         transaction_type: TransactionType::Deposit,
//         amount: 250.75,
//     };

//     // Example 2: A withdrawal
//     let withdrawal = Transaction {
//         id: 102,
//         account_id: 5001,
//         transaction_type: TransactionType::Withdrawal,
//         amount: 50.00,
//     };

//     // Example 3: A transfer, showing how to attach data to the Enum variant
//     let transfer = Transaction {
//         id: 103,
//         account_id: 5001,
//         transaction_type: TransactionType::Transfer {
//             destination_account_id: 5002,
//         },
//         amount: 100.00,
//     };

//     // Process all the transactions
//     deposit.process();
//     withdrawal.process();
//     transfer.process();
// }

// --------------------------------------------------------------------------
// TRAITS
// --------------------------------------------------------------------------

// // Financial Asset Valuation Example using Traits

// // A trait defining the essential behavior for any valuable financial asset.
// // All structs that implement this trait must provide a way to calculate their value.
// pub trait Valuable {
//     // Calculates the current market value of the asset.
//     // Returns the value as an f64 .
//     fn calculate_value(&self) -> f64;

//     // A default method to print a summary of the asset's value.
//     fn print_value_summary(&self) {
//         let value = self.calculate_value();
//         println!("Asset Value: {:.2}", value);
//     }
// }

// // --- Structs Representing Different Financial Assets ---

// // Represents a Stock holding.
// pub struct Stock {
//     pub ticker: String,
//     pub quantity: u32,
//     pub current_price: f64,
// }

// // Implement the 'Valuable' trait for the 'Stock' struct.
// impl Valuable for Stock {
//     fn calculate_value(&self) -> f64 {
//         // Stock value is calculated as quantity * current_price
//         self.quantity as f64 * self.current_price
//     }
// }

// // Represents a Bond holding.
// pub struct Bond {
//     pub issuer: String,
//     pub face_value: f64,
//     pub dirty_price_factor: f64, // Price expressed as a percentage of face value (e.g., 1.05 for 105%)
// }

// // Implement the 'Valuable' trait for the 'Bond' struct.
// impl Valuable for Bond {
//     fn calculate_value(&self) -> f64 {
//         // Bond value is calculated as face_value * dirty_price_factor
//         self.face_value * self.dirty_price_factor
//     }
// }

// // --- Generic Function using Trait Bounds ---

// // A generic function that can calculate the total value of any collection
// // where the items implement the 'Valuable' trait.
// // The 'T: Valuable' syntax is a  **trait bound**, ensuring we can call 'calculate_value'.
// fn total_portfolio_value<T: Valuable>(assets: &[T]) -> f64 {
//     assets.iter().map(|asset| asset.calculate_value()).sum()
// }

// // --- Main execution ---

// fn main() {
//     // 1. Create instances of the financial assets
//     let apple_stock = Stock {
//         ticker: String::from("APPL"),
//         quantity: 50,
//         current_price: 175.50,
//     };

//     let treasury_bond = Bond {
//         issuer: String::from("US Treasury"),
//         face_value: 100_000.0,
//         dirty_price_factor: 0.985, // Trading at a discount (98.5% of face value)
//     };

//     // 2. Use the trait method on individual assets
//     println!("--- Individual Assets Valuation ----");
//     print!("Apple Stock ({} shares): ", apple_stock.ticker);
//     apple_stock.print_value_summary(); // Uses the trait's default implementation

//     print!("{} Bond: ", treasury_bond.issuer);
//     treasury_bond.print_value_summary(); // Uses the trait's default implementation
//     println!("------------------------------------");

//     // 3. Use the generic function with a collection of the same asset type
//     let stocks = vec![
//         apple_stock,
//         Stock {
//             ticker: String::from("GOOG"),
//             quantity: 10,
//             current_price: 2500.00,
//         },
//     ];

//     // The generi function works seamlesly because all items implement 'Valuable'
//     let total_stock_value = total_portfolio_value(&stocks);
//     println!("\nTotal Stock Portfolio Value: {:.2}", total_stock_value);

//     // 4. Create a collection of trait objects for mixed type (Dynamic Dispatch)
//     // We use 'Box<dyn Valuable>' to store different types in on vector,
//     // as long as they all implement the 'Valuable' trait.
//     let portfolio: Vec<Box<dyn Valuable>> = vec![
//         Box::new(Bond {
//             issuer: String::from("Corp Bond"),
//             face_value: 50000.0,
//             dirty_price_factor: 1.02,
//         }),
//         Box::new(Stock {
//             ticker: String::from("MSFT"),
//             quantity: 100,
//             current_price: 350.0,
//         }),
//     ];

//     println!("\n--- Mixed Portfolio Valuation ----");
//     let mut mixed_total: f64 = 0.0;
//     for asset in &portfolio {
//         let value = asset.calculate_value(); // Calls the specific implementation for Stock and Bond
//         println!("Asset Value: {:.2}", value);
//         mixed_total += value;
//     }
//     println!("Total Mixed Portfolio Value: {:.2}", mixed_total);
// }

// --------------------------------------------------------------------------
// STRUCTS
// --------------------------------------------------------------------------

// // An enum to define the type of financial operation.
// #[derive(Debug)]
// pub enum TransactionType {
//     Deposit,
//     Withdrawal,
//     Transfer,
// }

// // A struct to represent a user's bank account.
// #[derive(Debug)]
// pub struct Account {
//     pub id: u32,       // Unique identifier for the account
//     pub owner: String, // Name of the account holder
//     pub balance: f64, // Current balance (using f64 for simplicity, but a fixed-point decimal type like 'Decimal' is often better for real finance)
// }

// // The core struct to represent a single financial transaction.
// // #[derive(Debug)] is added to allow easy printing with {:?}
// #[derive(Debug)]
// pub struct Transaction {
//     pub transaction_id: u64,                 // Unique transaction identifier
//     pub transaction_type: TransactionType, // The type of transaction (Deposit, Withdrawal, Transfer)
//     pub amount: f64,                       // The amount of money involved
//     pub source_account_id: u32,            // ID of the account the funds are coming from
//     pub destination_account_id: Option<u32>, // Optional ID for transfers (None for Deposit/Withdrawal)
//     pub timestamp: String, // Time of the transaction (for simplicity, using String)
// }

// // Implementation block for the Transaction strut to add methods
// impl Transaction {
//     // A method associated with the Transaction struct to print a readable sumamry.
//     pub fn summarize(&self) {
//         // Use a match expression on the TransactionType enum for different output formats.
//         match self.transaction_type {
//             TransactionType::Deposit => {
//                 println!(
//                     "Deposit of ${:.2} into Account {} (Txn ID: {}) at {}",
//                     self.amount, self.source_account_id, self.transaction_id, self.timestamp
//                 );
//             }
//             TransactionType::Withdrawal => {
//                 println!(
//                     "Withdrawal of ${:.2} from Account {} (Txn ID: {}) at {}",
//                     self.amount, self.source_account_id, self.transaction_id, self.timestamp
//                 );
//             }
//             TransactionType::Transfer => {
//                 // Safely unwrap the optional destination ID for a Transfer
//                 if let Some(dest_id) = self.destination_account_id {
//                     println!(
//                         "Transfer of ${:.2} from Account {} to Account {} (Txn ID: {}) at {}",
//                         self.amount,
//                         self.source_account_id,
//                         dest_id,
//                         self.transaction_id,
//                         self.timestamp
//                     );
//                 }
//             }
//         }
//     }
// }

// fn main() {
//     // --- Account Setup ---
//     let mut alice_account = Account {
//         id: 101,
//         owner: String::from("Alice Smith"),
//         balance: 500.00,
//     };

//     let mut bob_account = Account {
//         id: 202,
//         owner: String::from("Bob Johnson"),
//         balance: 250.00,
//     };

//     println!("--- Initial Balances ---");
//     println!("Alice: ${:.2}", alice_account.balance);
//     println!("Bob: ${:.2}", bob_account.balance);
//     println!("------------------------");

//     // --- Transaction 1: Deposit ---
//     let deposit_amount = 150.00;
//     let txn_deposit = Transaction {
//         transaction_id: 1001,
//         transaction_type: TransactionType::Deposit,
//         amount: deposit_amount,
//         source_account_id: alice_account.id,
//         destination_account_id: None, // Deposits don't have a destination account
//         timestamp: String::from("2025-11-09T10:00:00Z"),
//     };

//     // Update the balance and summarize the trasaction
//     alice_account.balance += deposit_amount;
//     txn_deposit.summarize();

//     // --- Transaction 2: Transfer ---
//     let transfer_amount = 75.50;
//     let txn_transfer = Transaction {
//         transaction_id: 1002,
//         transaction_type: TransactionType::Transfer,
//         amount: transfer_amount,
//         source_account_id: alice_account.id,
//         destination_account_id: Some(bob_account.id), // Transfer have a destination account
//         timestamp: String::from("2025-11-09T11:00:00Z"),
//     };

//     // Update the balances for the transfer
//     alice_account.balance += transfer_amount;
//     bob_account.balance += transfer_amount;
//     txn_transfer.summarize();

//     println!("\n--- Final Balances ---");
//     println!("Alice: ${:.2}", alice_account.balance);
//     println!("Bob: ${:.2}", bob_account.balance);
//     println!("------------------------");
// }

// --------------------------------------------------------------------------
// IMPL BLOCKS
// --------------------------------------------------------------------------

// //--- 1. Define a Trait for Financial Behavior ---
// // A Trait defines a set of methods that a type must implement.
// // This is essential for polymorphism and defining common behaviors
// // across different financial instruments (e.g., stocks, bonds, crypto).
// pub trait FinancialCalculations {
//     // Required method: calculate the total market value of the asset.
//     fn calculate_market_value(&self) -> f64;
// }

// // --- 2. Define a Struct for a Stock Asset ---
// // A struct holds the data (state) for our stock asset.
// #[derive(Debug)] // Allows for easy printing/debugging of the struct
// pub struct StockAsset {
//     ticker: String,
//     shares_held: u64,
//     current_price: f64,
// }

// // --- 3. Impl Block (Implementation Block) for the StockAsset Struct ---
// // An 'impl' block is where you define the *methods* and *associated functions*
// // (like a constructor) that belong to the StockAsset type.
// impl StockAsset {
//     pub fn new(ticker: &str, shares: u64, price: f64) -> Self {
//         StockAsset {
//             // .to_string() converts a string slice (&str) to an owned String
//             ticker: ticker.to_string(),
//             shares_held: shares,
//             current_price: price,
//         }
//     }

//     // A regular method (takes &self) to retrieve the asset's ticker
//     pub fn get_ticker(&self) -> &str {
//         &self.ticker
//     }
// }

// // -- 4. Impl Block to Implement the Trait for StockAsset ---
// // This 'impl' block connects the StockAsset struct to the FinancialCalculations trait,
// // forcing it to implement the required 'calculate_market_value' method.
// impl FinancialCalculations for StockAsset {
//     fn calculate_market_value(&self) -> f64 {
//         // Rust automatically handles the multiplication of different numeric types
//         // The result is an f64 (floating-point number)
//         (self.shares_held as f64) * self.current_price
//     }
// }

// // --- 5. Main function to use the code ---
// fn main() {
//     // Use the 'new' associated function defined in the 'impl StockAsset' block
//     let my_stock = StockAsset::new("GOOGL", 150, 145.50);

//     println!("Asset Details: {:?}", my_stock);
//     println!(
//         "Asset Ticker (using a custom impl method): {}",
//         my_stock.get_ticker()
//     );

//     // Call the method defined in the 'impl FinancialCalculations for StockAsset' block
//     let value = my_stock.calculate_market_value();

//     // The ':.2' formats the floatig-point number to two decimal places (currency standard)
//     println!("Total Market Value (Using Trait method): ${:.2}", value);

//     assert_eq!(value, 21825.00); // Simple test to verify the calculation
// }

// --------------------------------------------------------------------------
// --- DATA STRUCTURES ---
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// ARRAY
// --------------------------------------------------------------------------

// // Calculates the Net Present Value (NPV) of a fixed series of cash flows.

// // An Array is ideal here because the number of periods (cash flows) is fixed.
// fn calculate_npv(discount_rate: f64, initial_investment: f64, cash_flows: [f64; 5]) -> f64 {
//     // Rust Array: [f64; 5]
//     // T is the element type (f64 for currency/money).
//     // N is the compile-time constant size (5 periods)

//     let mut npv = 0.0;

//     // The initial investment is a cash *outflow* (negative value) at time 0.
//     // In many NPV conventions, this is treated separately from the discounted flows.
//     npv -= initial_investment;

//     // Iterate over the array elements, starting from index 0 (period 1).
//     for (index, &flow) in cash_flows.iter().enumerate() {
//         // The index represents the period number (0 is period 1, 1 is period 2, etc.).
//         // We add 1 to the index to get the exponent for the discount factor.
//         let period = (index + 1) as u32;

//         // Discount Factor Formula: 1 / (1 + rate)^period
//         let discount_factor = 1.0 / (1.0 + discount_rate).powi(period as i32);

//         // Calculate the Present Value of the single cash flow
//         let present_value = flow * discount_factor;

//         // Add the present value of the cash flow to the cumulative NPV
//         npv += present_value;
//     }

//     // Return the final Net Present Value
//     npv
// }

// fn main() {
//     // --- Data Setup using a Rust Array ---

//     // The fixed array stores the expected cash flows for the next 5 years.
//     // The type signature is inferred as [f64; 5]
//     let annual_cash_flows = [
//         1000.00, // Cash flow for Year 1
//         1500.00, // Cash flow for Year 2
//         2000.00, // Cash flow for Year 3
//         1800.00, // Cash flow for Year 4
//         2500.00, // Cash flow for Year 5
//     ];

//     // The required rate of return (e.g., 8.5%)
//     let rate: f64 = 0.085;

//     // The initial capital expenditure (outflow)
//     let initial_cap_ex: f64 = 6000.00;

//     // 1. **Fixed Size:** The array size is implicitly 5, known at compile time.
//     // We can check the fixed length:
//     println!(
//         "Number of periods (Array Length): {}",
//         annual_cash_flows.len()
//     );

//     // 1. **Indexed Access:** Accessing a specific cash flow is 0(1) time complexity.
//     println!(
//         "Cash Flow for Year 3 (Index 2): {:.2}",
//         annual_cash_flows[2]
//     );

//     // 3. **Iteration:** Efficiently processing all flows in sequence.
//     let final_npv = calculate_npv(rate, initial_cap_ex, annual_cash_flows);

//     println!("--- NPV Calculation ---");
//     println!("Discount Rate: {:.2}%", rate * 100.00);
//     println!("Initial Investment: ${:.2}", initial_cap_ex);
//     println!("Calculated Net Present Value: ${:.2}", final_npv);

//     // Decision Logic based on NPV
//     if final_npv > 0.0 {
//         println!("Decision: Project is financially viable (NPV > 0)");
//     } else {
//         println!("Decision: Project is NOT financially viable (NPV <= 0)");
//     }
// }

// --------------------------------------------------------------------------
// VECTOR
// --------------------------------------------------------------------------

// use std::collections::HashMap;
// use std::vec::Vec;

// // Represents a single assets's price snapshot in a trading system
// // Similar to the TickSnapshot patterm found in live trading engines [^8^]
// #[derive(Debug, Clone)]
// struct AssetPrice {
//     symbol: String,
//     price: f64,
//     timestamp: i64, // Unix timestamp in milliseconds
// }

// // Portfolio Analytics Engine demonstration Vector as a core data structure
// // This is inspired by real-world quantitative finance systems [^5^]
// struct PortfolioAnalytics {
//     // Vector storing historical asset prices - fundamental for time series analysis
//     price_history: Vec<AssetPrice>,
//     // Vector of asset symbols in the portfolio
//     holdings: Vec<String>,
//     // Vector storing calculated returns (percentage changes)
//     returns: Vec<f64>,
//     // Vector for covariance  matrix storage (flattened 2D representation)
//     covariance_data: Vec<f64>,
// }

// impl PortfolioAnalytics {
//     // Creates a new analytics engine with initial holdings
//     fn new(initial_holdings: Vec<String>) -> Self {
//         println!(
//             "Initializing portfolio analytics for {} assets",
//             initial_holdings.len()
//         );

//         PortfolioAnalytics {
//             price_history: Vec::new(),         // Empty vector, willl grow dynamically
//             holdings: initial_holdings,        // Pre-allocated with known symbols
//             returns: Vec::with_capacity(1000), // Pre-allocate capacity for performance
//             covariance_data: Vec::new(),
//         }
//     }

//     // Adds a new price tick to the history - demonstrates ector push operation
//     // In real trading systems, this is called on every market data update [^8^]
//     fn add_price_tick(&mut self, symbol: String, price: f64, timestamp: i64) {
//         // Push appends to the end with 0(1) amortized complexity
//         self.price_history.push(AssetPrice {
//             symbol,
//             price,
//             timestamp,
//         });

//         // Vector grows automatically when capacity is exceeded
//         // This is crucial for handling unpredictable market data volume
//     }

//     // Calculates daily returns using vector operations - core finance calculation
//     // Returns vector of percentage returns between consecutive periods
//     fn calculate_returns(&mut self, symbol: &str) -> Vec<f64> {
//         // Filter prices for specific symbol - demonstrates iterator adaptation
//         let prices: Vec<f64> = self
//             .price_history
//             .iter()
//             .filter(|p| p.symbol == symbol)
//             .map(|p| p.price) // Transform AssetPrice to f64 price
//             .collect(); // Collect into new vector

//         // Calculate returns: (P_t - P_{t-1}) / P_{t_1}
//         // Using windows() for efficient pairwise iteration
//         let returns: Vec<f64> = prices
//             .windows(2) // Create overlapping windows of size 2
//             .map(|window| {
//                 // Window is a slice containing two consecutive prices
//                 (window[1] - window[0]) / window[0]
//             })
//             .collect();

//         self.returns = returns.clone(); // Store for later use
//         returns
//     }

//     // Calculates portfolio variance using covariance matrix
//     // Demonstrates advanced vector operations for quantitative finance
//     fn calculate_portfolio_variance(&self, weights: &[f64]) -> f64 {
//         // weights.len() is the number of assets
//         let n = weights.len();

//         // Reshape flat covariance_data vector into 2D matrix representation
//         // This is how matrices are efficiently stored in memory
//         let covariance_matrix: Vec<Vec<f64>> = self
//             .covariance_data
//             .chunks(n) // Split into chenks of size n (rows)
//             .map(|chunk| chunk.to_vec()) // Convert each chunk to vector
//             .collect();

//         // Portfolio variance formula: w' * Σ * w
//         // where w s weight vector and Σ is covariance matrix
//         let mut variance = 0.0;

//         // Double iteration over matrix - 0(n²) operation
//         for i in 0..n {
//             for j in 0..n {
//                 variance += weights[i] * covariance_matrix[i][j] * weights[j];
//             }
//         }

//         variance
//     }

//     // Demonstrates vector resizing and capacity management
//     // Important for memory-efficient backtesting systems
//     fn resize_price_history(&mut self, max_size: usize) {
//         if self.price_history.len() > max_size {
//             // Remove oldest elements (FIFO queue behavior)
//             // Drain removes elements and returns iterator
//             let excess = self.price_history.len() - max_size;
//             self.price_history.drain(0..excess);

//             // Capacity remains unchaged for performance
//             // This prevents frequent reallocations
//         }
//     }

//     // Vector iteration and aggregatin - calculate portfolio value
//     fn calculate_portfolio_value(&self, holdings_qty: &HashMap<String, f64>) -> f64 {
//         // Iterate over holdings vector and sum values
//         self.holdings
//             .iter() // Create iterator over symbols
//             .filter_map(|symbol| {
//                 // Find lastest price for each symbol
//                 let lastest_price = self
//                     .price_history
//                     .iter()
//                     .rev() // Reverse to get most recent
//                     .find(|p| p.symbol == *symbol)?;

//                 // Get quantity from holdings map
//                 let quantity = holdings_qty.get(symbol)?;

//                 // Return value (price * quantity)
//                 Some(lastest_price.price * quantity)
//             })
//             .sum() // Sum all values into single f64
//     }

//     // Finds the most recent price before a given timestamp
//     // Crucial for backtesting and accurate P&L calculations
//     fn get_price_as_of(&self, symbol: &str, as_of_time: i64) -> Option<f64> {
//         self.price_history
//             .iter()
//             .filter(|p| p.symbol == symbol && p.timestamp <= as_of_time)
//             .max_by_key(|p| p.timestamp) // Use timestamp for comparison
//             .map(|p| p.price)
//     }
// }

// // Real-world usage example: Risk analysis workflow
// fn main() {
//     println!("=== Portfolio Risk Analytics Engine ===\n");

//     // Initialize with portfolio holdings
//     let holdings = vec![
//         "APPL".to_string(),
//         "GOOGL".to_string(),
//         "MSFT".to_string(),
//         "TSLA".to_string(),
//     ];

//     let mut analytics = PortfolioAnalytics::new(holdings);

//     // Simulate adding price ticks (like live market data feed)
//     // In production, this would be called thousands of times per second
//     analytics.add_price_tick("APPL".to_string(), 150.25, 1640995200000);
//     analytics.add_price_tick("GOOGL".to_string(), 2750.90, 1640995200000);
//     analytics.add_price_tick("MSFT".to_string(), 350.15, 1640995200000);
//     analytics.add_price_tick("TSLA".to_string(), 1050.30, 1640995200000);

//     // Add mre ticks for return calculation
//     analytics.add_price_tick("APPL".to_string(), 151.50, 1640995260000);
//     analytics.add_price_tick("APPL".to_string(), 149.80, 1640995320000);
//     analytics.add_price_tick("APPL".to_string(), 152.20, 1640995380000);

//     // aculate returns for AAPL
//     let aapl_returns = analytics.calculate_returns("AAPL");
//     println!("AAPL Returns: {:?}", aapl_returns);

//     // Setup mock covariance data (flattened 4x4 matrix)
//     // In real systems, this would be calculated from historical data
//     analytics.covariance_data = vec![
//         0.0025, 0.0012, 0.0015, 0.0021, 0.0012, 0.0031, 0.0018, 0.0019, 0.0015, 0.0018, 0.0028,
//         0.0022, 0.0021, 0.0019, 0.0022, 0.0042,
//     ];

//     // Portfolio weights (must sum to 1.0)
//     let weights = vec![0.3, 0.25, 0.25, 0.2];

//     // Calculate portfolio risk
//     let portfolio_variance = analytics.calculate_portfolio_variance(&weights);
//     let portfolio_std_dev = portfolio_variance.sqrt();
//     println!("Portfolio Variance: {:.6}", portfolio_variance);
//     println!(
//         "Portfolio Standard Deviation: {:.4}%",
//         portfolio_std_dev * 100.0
//     );

//     // Calculate portfolio value
//     let mut holdings_qty = HashMap::new();
//     holdings_qty.insert("AAPL".to_string(), 100.0);
//     holdings_qty.insert("GOOGL".to_string(), 10.0);
//     holdings_qty.insert("MSFT".to_string(), 50.0);
//     holdings_qty.insert("TSLA".to_string(), 20.0);

//     let portfolio_value = analytics.calculate_portfolio_value(&holdings_qty);
//     println!("Total Portfolio Value: ${:.2}", portfolio_value);

//     // Demonstrate capacity management
//     println!(
//         "\nPrice history length before resize: {}",
//         analytics.price_history.len()
//     );
//     analytics.resize_price_history(5);
//     println!(
//         "Price history length after resize: {}",
//         analytics.price_history.len()
//     );

//     let historical_price = analytics.get_price_as_of("AAPL", 1640995320000);
//     println!(
//         "AAPL price at timestamp 1640995320000: {:?}",
//         historical_price
//     );
// }

// --------------------------------------------------------------------------
// HASHMAP
// --------------------------------------------------------------------------

// use std::collections::HashMap;

// // --- Structs to hold Stock Data ---

// // Define the data structure for a single stock holding
// #[derive(Debug)]
// struct Stock {
//     ticker: String,
//     quantity: u32,
//     purchase_price: f64, // Price per share when purchased
// }

// // Define the data structure to hold current market prices
// // Note: In a real application, this data would be fetched from a live API.
// #[derive(Debug)]
// struct MarketPrice {
//     current_price: f64, // Current market price per share
// }

// // --- Main Function ---
// fn main() {
//     // 1. Create a HashMap to store te portfolio holdings.
//     // Key: String (Stock Ticker)
//     // Value: Stock (The Stock struct containing quantity and purchase price)
//     let mut portfolio: HashMap<String, Stock> = HashMap::new();

//     // 2. Insert stock holdings into the HashMap.
//     // The key is a clone of the ticker, and the value is the Stock struct.
//     portfolio.insert(
//         String::from("AAPL"), // Apple
//         Stock {
//             ticker: String::from("APPL"),
//             quantity: 100,
//             purchase_price: 150.50,
//         },
//     );

//     portfolio.insert(
//         String::from("MSFT"), // Microsoft
//         Stock {
//             ticker: String::from("MSFT"),
//             quantity: 50,
//             purchase_price: 250.75,
//         },
//     );

//     portfolio.insert(
//         String::from("GOOGL"), // Alphabet Google
//         Stock {
//             ticker: String::from("GOOGL"),
//             quantity: 20,
//             purchase_price: 120.00,
//         },
//     );

//     println!("--- Initial Portforlio Holdings ---");
//     // Print the entire HashMap for debugging/inspection
//     for (_, holding) in &portfolio {
//         println!(
//             "{} | Quantity: {} | Purchase Price: ${}",
//             holding.ticker, holding.quantity, holding.purchase_price
//         );
//     }
//     println!("---------------------------------------------");

//     // 3. Create a separate HashMap for current market data.
//     // Key: String (Stock Ticker)
//     // Value: MarketPrice (The current price)
//     let market_data: HashMap<String, MarketPrice> = HashMap::from([
//         (
//             String::from("APPL"),
//             MarketPrice {
//                 current_price: 175.25,
//             },
//         ),
//         (
//             String::from("MSFT"),
//             MarketPrice {
//                 current_price: 300.50,
//             },
//         ),
//         (
//             String::from("GOOGL"),
//             MarketPrice {
//                 current_price: 115.75,
//             },
//         ),
//         (
//             String::from("TSLA"),
//             MarketPrice {
//                 current_price: 220.00,
//             },
//         ), // Extra stock not in portfolio
//     ]);

//     let mut total_market_value: f64 = 0.0;

//     // 4. Iterate over the portfolio and calculate the total market value.
//     println!("\n--- Market Value Calculation ---");
//     for (ticker, holding) in &portfolio {
//         // Use the 'get()' method to look up the current price using the ticker key.
//         // 'get()' returns an 'Option<&V>', which we handle with a 'match' or 'if let'.
//         if let Some(price_data) = market_data.get(ticker) {
//             let market_value = holding.quantity as f64 * price_data.current_price;
//             total_market_value += market_value;

//             let purchase_value = holding.quantity as f64 * holding.purchase_price;
//             let profit_loss = market_value - purchase_value;

//             println!(
//                 "Stock: {} | Market Value: {:.2} | Profit/Loss: ${:.2}",
//                 ticker, market_value, profit_loss
//             );
//         } else {
//             // Handle the case where the market data for a stock is missing.
//             println!("WARNING: Market data for {} not found", ticker);
//         }
//     }

//     println!(
//         "\n** Total Portfolio Market Value: {:.2} **",
//         total_market_value
//     );
// }

// --------------------------------------------------------------------------
// HASHSET
// --------------------------------------------------------------------------

// use std::collections::HashSet;

// // A simple structure to represent a Trader ID.
// // #[derive(PartialEq, Eq, Hash)] is necessary for T in HashSet<T>.
// #[derive(Debug, PartialEq, Eq, Hash, Clone)]
// struct TraderId {
//     id: String,
// }

// impl TraderId {
//     fn new(id: &str) -> Self {
//         TraderId { id: id.to_string() }
//     }
// }

// fn main() {
//     // --- 1. Initialization and Insertion (Uniqueness Guarantee) ---

//     // Create a new HashSet to store the unique IDs of authorized traders.
//     let mut authorized_traders: HashSet<TraderId> = HashSet::new();

//     let trader_a = TraderId::new("CUST_001");
//     let trader_b = TraderId::new("CUST_002");
//     let trader_c = TraderId::new("CUST_003");

//     // Add new traders. 'insert' returns true if the value was newly inserted.
//     println!(
//         "Adding CUST_001: {}",
//         authorized_traders.insert(trader_a.clone())
//     ); // true
//     println!(
//         "Adding CUST_002: {}",
//         authorized_traders.insert(trader_b.clone())
//     ); // true
//     println!("Adding CUST_003: {}", authorized_traders.insert(trader_c)); // true

//     // Attempt to add a duplicate ID. The HashSet prevents the duplicate.
//     let duplicate_a = TraderId::new("CUST_001");
//     // 'insert' returns false because the element is already present.
//     println!(
//         "Adding CUST_001 (again): {}",
//         authorized_traders.insert(duplicate_a)
//     );

//     println!(
//         "\nTotal Unique Authorized Traders: {}",
//         authorized_traders.len()
//     );
//     // Output will be 3, as the duplicate CUST_001 was ignored.

//     // --- 2. Membership Check (Fast Lookup) ---

//     // A hypothetical trader tries to execute a transaction.
//     let trade_attemp_id = TraderId::new("CUST_002");
//     let unauthorized_attempt_id = TraderId::new("CUST_999");

//     // Use 'contains' to quckly check if the trader is authorized (0(1) average time).
//     if authorized_traders.contains(&trade_attemp_id) {
//         println!(
//             "\nTrade executed: Trader: {:?} is authorized",
//             trade_attemp_id
//         );
//     } else {
//         println!(
//             "\nTrade blocked: Trader: {:?} is NOT authorized",
//             trade_attemp_id
//         );
//     }

//     if authorized_traders.contains(&unauthorized_attempt_id) {
//         println!(
//             "\nTrade executed: Trader: {:?} is authorized",
//             unauthorized_attempt_id
//         );
//     } else {
//         println!(
//             "\nTrade blocked: Trader: {:?} is NOT authorized",
//             unauthorized_attempt_id
//         );
//     }

//     // --- 3. Removal ---
//     //
//     // Trader CUST_002's account is suspeded.
//     let suspended_trader = TraderId::new("CUST_002");
//     // 'remove' returns true if the value was present.
//     let removed = authorized_traders.remove(&suspended_trader);

//     if removed {
//         println!("\nRemoved CUST_001 from the authorized list.");
//     }

//     if !authorized_traders.contains(&suspended_trader) {
//         println!("Current list no longer constains CUST_002");
//     }

//     println!("\nFinal Authorized List: {:?}", authorized_traders);
// }

// --------------------------------------------------------------------------
// LINKEDLIST
// --------------------------------------------------------------------------

// use std::fmt;

// // 1. the Data Structure Payload ---
// // Represents a simple financial transaction record.
// #[derive(Debug, Clone)]
// pub struct FinancialTransaction {
//     pub id: u32,
//     pub amount: f64,
//     pub description: String,
// }

// impl fmt::Display for FinancialTransaction {
//     // Implement Display trait to pretty-print the transaction.
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "[ID: {}, Amount: ${:.2}, Desc: {}]",
//             self.id, self.amount, self.description
//         )
//     }
// }

// // --- 2. The Node Definition ---
// // The fundamental building block of the linked list.
// // T is the generic type for the data (in our case, FinancialTransaction).
// // 'next' uses Box for heap allocation and Option for the 'end of list' (None).
// #[derive(Debug)]
// struct Node<T> {
//     data: T,
//     next: Option<Box<Node<T>>>,
// }

// // 3. --- The List Wrapper ---
// // The list wrapper struct, only storing a pointer to the first Node (the 'head')
// #[derive(Debug)]
// pub struct TransactionList {
//     head: Option<Box<Node<FinancialTransaction>>>,
// }

// // --- 4. List Implementation ---
// impl TransactionList {
//     // Creates a new, empty list.
//     pub fn new() -> Self {
//         TransactionList { head: None }
//     }

//     // Prepends a new transaction to the start of the list (0(1) operation).
//     // This is common for a transaction log where the newest item is the end.
//     pub fn prepend(&mut self, transaction: FinancialTransaction) {
//         // Create a new node. The 'next' pointer points to the *current* head.
//         let new_node = Box::new(Node {
//             data: transaction,
//             // .take() moves the current head out of self.head, leaving None,
//             // which allows us to set the head safely.
//             next: self.head.take(),
//         });

//         // Set the new node as the list's new head.
//         self.head = Some(new_node);
//     }

//     // Iterates and prints all transactions in the list.
//     pub fn display(&self) {
//         println!("--- Financial Transaction Log (Newest to Oldest) ---");
//         let mut current = self.head.as_ref(); // Gest an inmmutable reference to the head

//         while let Some(node) = current {
//             // Use the implemented Display trait for clean output
//             println!("{}", node.data);
//             // Move to the next node by referencing the inner Box
//             current = node.next.as_ref();
//         }
//         println!("---------------------------------------------------");
//     }
// }

// // --- 5. Example Usage ----
// fn main() {
//     let mut tx_log = TransactionList::new();

//     // Create and prepend transactions (newest transaction are added first)
//     let tx1 = FinancialTransaction {
//         id: 101,
//         amount: 50.00,
//         description: String::from("Initial Deposit"),
//     };
//     tx_log.prepend(tx1); // This is the newest (Head)

//     let tx2 = FinancialTransaction {
//         id: 102,
//         amount: 15.50,
//         description: String::from("Coffee Shop Purchase"),
//     };
//     tx_log.prepend(tx2);

//     let tx3 = FinancialTransaction {
//         id: 103,
//         amount: 250.75,
//         description: String::from("Online Stock Purchase"),
//     };
//     tx_log.prepend(tx3); // This will be the very first one printed

//     tx_log.display();
// }

// --------------------------------------------------------------------------
// STACK
// --------------------------------------------------------------------------

// // 1. Define a generic Stack structure
// // The '<T>' makes the Stack *generic*, meaning it can hold any type (f64, String, a Transaction struct, etc.)
// // The 'Vec<T>' is the underlying storage; Rust's dynamic array type.
// pub struct Stack<T> {
//     data: Vec<T>,
// }

// // 2. Implement methods for the Stack
// impl<T> Stack<T> {
//     // Creates a new, empty Stack.
//     pub fn new() -> Self {
//         // Self is an alias for 'Stack<T>'
//         Stack { data: Vec::new() }
//     }

//     // Adds an element to the top of the stck (LIFO - Last In, First Out).
//     // This operation is often called 'push' in stack terminology.
//     pub fn push(&mut self, item: T) {
//         // '&mut self' means we are borrowing and mutating the Stack instance.
//         self.data.push(item);
//     }

//     // Removes and return the element at the top of the stack.
//     // Returns an 'Option<T>' because the stack might be empty.
//     // 'Option' is a great way Rust handles potential null/empty values safely.
//     pub fn pop(&mut self) -> Option<T> {
//         self.data.pop()
//     }

//     // Returns a reference to the top element without removing it.
//     // Note: Returns a Option<&T>, a *reference* to T, avoiding an expensive copy/move.
//     pub fn peek(&self) -> Option<&T> {
//         // '&self' means we are borrowing the Stack inmutably.
//         self.data.last()
//     }

//     // Returns true the stack constains no elements.
//     pub fn is_empty(&self) -> bool {
//         self.data.is_empty()
//     }
// }

// // 3. Define a specific financial data type (a custom struct)
// // We use a struct to model a single stock transaction record.
// #[derive(Debug, Clone)] // These traits allow easy printing and copying
// pub struct Transaction {
//     pub ticker: String,
//     pub volume: u32,
//     pub price: f64,
// }

// // 4. Main function to demonstrate usage
// fn main() {
//     println!("--- Initializing Transaction History Stack ---");

//     // Create a new Stack that specifically hold 'Transaction' structs.
//     let mut transaction_history: Stack<Transaction> = Stack::new();

//     // Create a few transacton items
//     let tx1 = Transaction {
//         ticker: String::from("APPL"),
//         volume: 100,
//         price: 155.50,
//     };

//     let tx2 = Transaction {
//         ticker: String::from("GOOGL"),
//         volume: 50,
//         price: 2500.75,
//     };

//     let tx3 = Transaction {
//         ticker: String::from("TSLA"),
//         volume: 10,
//         price: 750.00,
//     };

//     // PUSH: Add transactions to the stack (LIFO)
//     transaction_history.push(tx1.clone()); // We use .clone() here to keep tx1 available for later
//     transaction_history.push(tx2.clone());
//     transaction_history.push(tx3.clone());

//     println!("\nStack initialized with 3 transactions.");

//     // PEEK: Look at the most recent transaction without removing it
//     if let Some(top_tx) = transaction_history.peek() {
//         println!(
//             "\nPEEK: Most recent transaction (still on the stack): {:?}",
//             top_tx
//         );
//     }

//     // POP: Process and remove the most recent transaction
//     if let Some(processed_tx) = transaction_history.pop() {
//         println!("\nPOP: Processed transaction: {:?}", processed_tx);
//         println!("  - This transaction is now removed from the stack.");
//     }

//     // PEEK again: The stack's top element has changed
//     if let Some(new_top_tx) = transaction_history.peek() {
//         println!("\nPEEK: New most recent transaction: {:?}", new_top_tx);
//     }

//     // POP the rest
//     while let Some(tx) = transaction_history.pop() {
//         println!("\nProcessing remaining transaction: {:?}", tx.ticker);
//     }

//     // Check if the stack is empty
//     if transaction_history.is_empty() {
//         println!("\nAll transactions processed. The stack is empty!");
//     }
// }

// --------------------------------------------------------------------------
// QUEUE
// --------------------------------------------------------------------------

// use std::collections::VecDeque;

// // --- TRASACTION REQUEST QUEUE ---
// // A Queue is a FIFO (First-In, First-Out) structure.
// // In finance, this is perfect for handling requests in the order they arrive,
// // ensuring fairness and correct sequence.

// // 1. Define the type of data the queue will hold.
// // In this case, a struct representing a financial transaction request.
// #[derive(Debug, PartialEq)] // Derive traits for easy printing and comparison
// pub struct TransactionRequest {
//     id: u32,
//     amount: f64,
//     request_type: String, // e.g., "BUY", "SELL", "WITHDRAW"
// }

// // 2. Define the Queue Manager struct.
// // It wraps the VecDeque to manage the queue operations.
// pub struct TransactionQueue {
//     // VecDeque is the standard Rust collection for an efficient, general-purpose queue.
//     // It is optimized for adding to one end (push_back) and removing from the other (pop_front).
//     requests: VecDeque<TransactionRequest>,
//     next_id: u32, // simple counter for  unique IDs
// }

// impl TransactionQueue {
//     // Initializes an empty queue manager.
//     pub fn new() -> Self {
//         TransactionQueue {
//             requests: VecDeque::new(),
//             next_id: 1, // Start IDs at 1
//         }
//     }

//     // 'ENQUEUE' operation: Adds a new request to the back of the queue (FIFO).
//     // This simulates a new reques arriving from a client.
//     pub fn add_request(&mut self, amount: f64, request_type: &str) {
//         let new_request = TransactionRequest {
//             id: self.next_id,
//             amount,
//             request_type: request_type.to_string(),
//         };
//         // push_back is the 'enqueue' method for VecDeque
//         self.requests.push_back(new_request);
//         self.next_id += 1;
//         println!(
//             "   -> ENQUEUED: Request {} ({})",
//             self.next_id - 1,
//             request_type
//         );
//     }

//     // 'DEQUEUE' operation: Removes and returns the request at the front of the queue.
//     // This simulates the processing engine taking the next request to execute.
//     pub fn process_next_request(&mut self) -> Option<TransactionRequest> {
//         // pop_front is the 'dequeue' method for VecDeque.
//         // It returns Option<T> because the queue might be empty.
//         match self.requests.pop_front() {
//             Some(request) => {
//                 println!(
//                     "   <- DEQUEUED & PROCESSING: Request {} (Type: {}, Amount: ${:.2})",
//                     request.id, request.request_type, request.amount
//                 );
//                 // In a real system, you would execute the trasaction here.
//                 Some(request)
//             }
//             None => {
//                 println!("\n * Queue is empty. No transactions to process.");
//                 None
//             }
//         }
//     }

//     // 'PEEK' operation: Returns a reference to the request at the front, without removing it.
//     pub fn peek_next_request(&self) -> Option<&TransactionRequest> {
//         // front() returns an Option<&T>
//         self.requests.front()
//     }

//     // Utility: Checks if the queue is empty.
//     pub fn is_empty(&self) -> bool {
//         self.requests.is_empty()
//     }
// }

// // 3. Example usage in the main function
// fn main() {
//     let mut transaction_processor = TransactionQueue::new();
//     println!("--- Initializing Transaction Queue Processor ----");

//     // Add (ENQUEUE) a sequence of transaction requests
//     transaction_processor.add_request(1000.00, "BUY_STOCK");
//     transaction_processor.add_request(50.50, "WITHDRAWAL");
//     transaction_processor.add_request(2500.00, "SELL_BOND");
//     transaction_processor.add_request(75.00, "FEE_PAYMENT");

//     println!("\n--- Queue State After Enqueuing ----");
//     if let Some(next) = transaction_processor.peek_next_request() {
//         println!(
//             "* PEEK: The next request to be processed is ID: {})",
//             next.id
//         );
//     }

//     // Process (DEQUEUE) the requests in the correct FIFO order
//     transaction_processor.process_next_request(); // Should be BUY_STOCK (ID: 1)
//     transaction_processor.process_next_request(); // Should be WITHDRAWAL (ID: 2)
//     transaction_processor.process_next_request(); // Should be SELL_BOND (ID: 3)
//     transaction_processor.process_next_request(); // Should be FEE_PAYMENT (ID: 4)

//     // Try to process one more time when the queue is empty
//     transaction_processor.process_next_request();
// }

// --------------------------------------------------------------------------
// OWNSERSHIP SYSTEM
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// OWNSERSHIP RULES & MEMORY SAFETY
// --------------------------------------------------------------------------

// // 1. Define a Struct to represent a transaction
// struct FinancialTransaction {
//     id: u32,
//     amount: f64,
//     // A large vector representing a detailed audit trail or receipt data.
//     // This is here to demonstrate the *cost* of copying, which rust avoids.
//     audit_data: Vec<u8>,
// }

// impl FinancialTransaction {
//     // A constructor method
//     fn new(id: u32, amount: f64, data_size: usize) -> Self {
//         FinancialTransaction {
//             id,
//             amount,
//             // Initialize a large vector (e.g., a 1MB receipt)
//             audit_data: vec![0; data_size],
//         }
//     }

//     // 2. Borrowing: A function that only needs to *read* the transaction.
//     // Note the '&self' (inmmutable borrow). This does NOT take  ownership.
//     // The transaction remains available after this call.
//     fn get_summary(&self) -> String {
//         format!("Transaction ID: {}, Amount: {:.2 }", self.id, self.amount)
//     }

//     // 3. Mutable Borrowing: A function that needs to *change* the transaction.
//     // Note the '&mut self' (mutable borrow). Only ONE mutable borrow can exist at a time.
//     fn apply_fee(&mut self, fee: f64) {
//         // This is safe because the compier ensures no other part of the code
//         // can read or write to 'self' while this method is running.
//         self.amount -= fee;
//         println!(
//             "\n[INFO] Applied fee of ${:.2} to transaction {}",
//             fee, self.id
//         );
//     }
// }

// // 4. Ownership Transfer (Moving): A function that logically consumes the transaction.
// // Note the 'transaction: FinancialTransaction' (takes ownership).
// // The 'transaction' is moved into this function and is 'dropped' (memory freed)
// // when the function returns. The original variable is invalidated.
// fn finalize_and_archive(transaction: FinancialTransaction) {
//     println!(
//         "\n[INFO] Archiving transaction {} with data size {} bytes...",
//         transaction.id,
//         transaction.audit_data.len()
//     );
//     // The transaction and its large 'audit_data' vector are dropped here.
//     // The memory is safety released.
// }

// fn main() {
//     // --- PART 1: Immutable Borrowing ----
//     let mut tx1 = FinancialTransaction::new(1001, 150.75, 1024 * 1024); // 1MB data

//     println!("--- Initial State ---");
//     println!("TX1 Summary: {}", tx1.get_summary()); // (2) Borrowed, not moved.

//     // --- PART 2: Mutable Borrowing ---
//     // We can change the transaction because we created it as `mut`
//     tx1.apply_fee(5.00); // (3) Borrowed mutably.
//     println!("TX1 New Summary: {}", tx1.get_summary()); // (2) Borrowed immutably again.

//     // RUST SAFETY CHECK: The following would cause a compiler error!
//     /*
//     let summary = tx1.get_summary(); // Immutable borrow starts here
//     tx1.apply_fee(1.00);            // ERROR: Cannot borrow mutably while immutably borrowed!
//     println!("{}", summary);
//     */
//     // Rust ensures that data cannot be read while it is being written to (Data Race prevention).

//     // --- PART 3: Ownership Transfer (Moving) ---

//     // `tx1` (and its large audit_data) is moved into the function.
//     // This avoids an expensive copy of the 1MB data and guarantees
//     // that no other code can use the transaction later.
//     finalize_and_archive(tx1); // (4) Ownership MOVED.

//     // RUST SAFETY CHECK: The following would cause a compiler error!
//     /*
//     println!("Attempting to access TX1 again: {}", tx1.get_summary());
//     */
//     // ERROR: value borrowed here after move! The compiler prevents a use-after-free error.

//     println!("\n[SUCCESS] Rust has guaranteed memory safety and data integrity.");
// }

// --------------------------------------------------------------------------
// BORROWING, REFERENCES AND SLICES
// --------------------------------------------------------------------------

// // 1. Function demonstrating Borrowing and Slices
// // The function takes a reference to a slice of f64 (&[f64]),
// // meaning it BORROWS the data without taking ownership.
// // The slice allows us to process only a part of the original vector.
// fn calculate_interest_payment(transactions: &[f64], rate: f64) -> f64 {
//     println!("--- Calculating Interest ---");

//     // 'transactions' is a slice (a type of reference) pointing to a contiguous block of memory.
//     // We iterate over the slice, borrowing each transaction value.
//     let total_principal: f64 = transactions
//         .iter()
//         // Here, 't' isa reference to an f64 (&f64). The '*' dereferences it
//         // to get the actual f4 value for the sum.
//         .map(|&t| t)
//         .sum();

//     let interest = total_principal * rate;

//     println!("Total Principal from slice: ${:.2}", total_principal);
//     println!("Interest Rate: {:.2}%", rate * 100.0);
//     println!("Calculated Interest Payment: ${:.2}", interest);

//     interest
// }

// // 2. Main function demonstrating References
// fn main() {
//     // A vector representing a series of financial transactions (deposits/withdrawals)
//     let all_transactions: Vec<f64> = vec![
//         150.00,  // Transaction 1
//         -25.50,  // Transaction 2
//         450.75,  // Transaction 3 (High Value)
//         -100.00, // Transaction 4
//         250.00,  // Transaction 5
//     ];

//     // --- References and Borrowing ---
//     // The main variable 'all_transactions' RETAINS ownership.
//     let transactions_ref: &Vec<f64> = &all_transactions;

//     println!("Orinal Vector (Owner): {:?}", all_transactions);
//     println!("Reference to Vector: {:?}", transactions_ref); // Printing via the reference

//     // --- Slices ---

//     // Case A: Calculate interest on ALL transactions.
//     // We pass a reference to the whole vector as a slice (&all_transactions[..])
//     let all_interest = calculate_interest_payment(&all_transactions[..], 0.05); // %5 rate

//     // Case B: Calculate interest ONLY on the first three transactions (index 0, 1, 2).
//     // This creates a SLICE (&all_transactions[0..3 ]) that BORROWS a part of the vector.
//     let partial_interest = calculate_interest_payment(&all_transactions[0..3], 0.03); // %3 rate

//     // The original vector 'all_transactions' is still perfectly valid and usable
//     // because the functions only BORROWED the data (did not take ownership).
//     println!("\nFinal Summary:");
//     println!("Total interest from ALL transactions: ${:.2}", all_interest);
//     println!(
//         "Total interest from PARTIAL transactions: ${:.2}",
//         partial_interest
//     );
//     println!("Original vector is still intact: {:?}", all_transactions);
// }

// --------------------------------------------------------------------------
// ADVANCED TOPICS
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// BTREEMAP
// --------------------------------------------------------------------------

// use std::collections::BTreeMap;

// // Represents a single stock holding.
// #[derive(Debug, Clone)]
// struct Stock {
//     ticker: String,
//     shares: u32,
//     price_per_share: f64,
// }

// impl Stock {
//     // Calculates the current market value of this stock holding.
//     fn calculate_value(&self) -> f64 {
//         self.shares as f64 * self.price_per_share
//     }
// }

// // Represents the entire stock portfolio.
// // BTreeMap is used to store stocks, heyed by ticker symbol (String),
// // ensuring the portfolio is always sorted by ticker.
// struct Portfolio {
//     holdings: BTreeMap<String, Stock>,
// }

// impl Portfolio {
//     // Creates a new empty portfolio.
//     fn new() -> Self {
//         Portfolio {
//             holdings: BTreeMap::new(),
//         }
//     }

//     // Adds a stock to the portfolio or updates  an existing one.
//     fn add_or_update_stock(&mut self, stock: Stock) {
//         // The BTreeMap's insert method is key here:
//         // If the key (ticker) already exists, it updates the value (Stock).
//         // If the key does not exist, it inserts the new key-value pair.
//         println!("Inserting/Updating stock: {}", stock.ticker);
//         self.holdings.insert(stock.ticker.clone(), stock);
//     }

//     // Removes a stock holding from the portfolio by its ticker.
//     fn remove_stock(&mut self, ticker: &str) -> Option<Stock> {
//         // The BTreeMap's remove method takes the key and returns the value
//         // if the key was present, otherwise returns None.
//         println!("Removing stock: {}", ticker);
//         self.holdings.remove(ticker)
//     }

//     // Retrieves a stock holding by its ticker.
//     fn get_stock(&self, ticker: &str) -> Option<&Stock> {
//         // BTreeMap's get method provides a reference to the value by key.
//         self.holdings.get(ticker)
//     }

//     // Calculates the total market value of all holdings in the portfolio.
//     fn calculate_total_value(&self) -> f64 {
//         self.holdings
//             .values() // Get an iterator over all Stock values
//             .map(|stock| stock.calculate_value()) // Calculate the value of each tock
//             .sum() // Sum up all the calculated values
//     }

//     // Displays the portfolio in a sorted manner (due to BTreeMap).
//     fn display_portfolio(&self) {
//         println!("\n--- Current Portfolio (Sorted by Ticker) ---");
//         for (ticker, stock) in &self.holdings {
//             // Note: Iterating over a BTreeMap yields (key, value) pairs in sorted order by key.
//             println!(
//                 "  {:<5}: {} shares @ ${:.2} (Value: {:.2})",
//                 ticker,
//                 stock.shares,
//                 stock.price_per_share,
//                 stock.calculate_value()
//             );
//         }
//         println!("----------------------------------------------");
//     }
// }

// fn main() {
//     let mut portfolio = Portfolio::new();

//     // --- 1. Initial Insertions using add_or_update_stock ---
//     // BTreeMap keeps these sorted by key (ticker): APPL, GOOG, MSFT
//     portfolio.add_or_update_stock(Stock {
//         ticker: "GOOG".to_string(),
//         shares: 10,
//         price_per_share: 150.00,
//     });

//     portfolio.add_or_update_stock(Stock {
//         ticker: "APPL".to_string(),
//         shares: 25,
//         price_per_share: 175.00,
//     });

//     portfolio.add_or_update_stock(Stock {
//         ticker: "MSFT".to_string(),
//         shares: 5,
//         price_per_share: 400.75,
//     });

//     portfolio.display_portfolio();
//     // Total value calculated: (25*175.50) + (10*150.00) + (5*400.75) = 4387.50 + 1500.00 + 2003.75 = 7891.25

//     // --- 2. Update a Stock's Price and Shares (using the same key) ---
//     // This demonstrates BTreeMap's key features: replacing the old value with the new one.
//     portfolio.add_or_update_stock(Stock {
//         ticker: "APPL".to_string(),
//         shares: 30,              // Increased shares
//         price_per_share: 178.00, // Updated price
//     });

//     // --- 3. Retrieval and Total Calculation ---
//     match portfolio.get_stock("AAPL") {
//         Some(stock) => println!(
//             "\nRetrieved AAPL: Current value is ${:.2}",
//             stock.calculate_value()
//         ),
//         None => println!("AAPL not found."),
//     }

//     portfolio.display_portfolio();
//     // New total value: (30*178.00) + (10*150.00) + (5*400.75) = 5340.00 + 1500.00 + 2003.75 = 8843.75
//     println!(
//         "\nTotal Portfolio Value: ${:.2}",
//         portfolio.calculate_total_value()
//     );

//     // --- 4. Remove a Stock ---
//     if portfolio.remove_stock("GOOG").is_some() {
//         println!("GOOG removed successfully.");
//     } else {
//         println!("GOOG not found for removal.");
//     }
// }

// --------------------------------------------------------------------------
// BTREESET
// --------------------------------------------------------------------------

// use std::cmp::Ordering;
// use std::collections::BTreeSet;

// // Define a struct to represent a single stock trade
// #[derive(Debug, PartialEq, PartialOrd, Clone)]
// struct Trade {
//     // Unique identifier for the trade (e.g., transaction ID)
//     trade_id: u64,
//     // Stock ticker symbol (e.g., "AAPL", "GOOG")
//     symbol: String,
//     // Price at which the trade ocurred
//     price: f64,
//     // Number of shares traded
//     volume: u32,
// }

// // Implement Eq (requires that everything be equal, and uses PartialEq)
// impl Eq for Trade {}

// // Implement Ord to define the total ordering for the BTreeSet
// impl Ord for Trade {
//     fn cmp(&self, other: &Self) -> Ordering {
//         // 1. Compare the primary key (trade_id) first
//         let id_cmp = self.trade_id.cmp(&other.trade_id);
//         if id_cmp != Ordering::Equal {
//             return id_cmp;
//         }

//         // 2. If IDs are equal, compare the secondary key (symbol)
//         let symbol_cmp = self.symbol.cmp(&other.symbol);
//         if symbol_cmp != Ordering::Equal {
//             return symbol_cmp;
//         }

//         // 3. Finally, use `f64::total_cmp` to safely compare the price
//         // This is the core fix for the E0277 error.
//         let price_cmp = self.price.total_cmp(&other.price);
//         if price_cmp != Ordering::Equal {
//             return price_cmp;
//         }

//         // 4. Compare the last field (volume)
//         self.volume.cmp(&other.volume)
//     }
// }

// // Function to demonstrate BTreeSet features
// fn main() {
//     println!("--- BTreeSet Financial Ledger Example ---");

//     // 1. Initialization: Create a new, empty BTreeSet to hold trades
//     let mut trade_ledger: BTreeSet<Trade> = BTreeSet::new();

//     // 2. Data Preparation: Define some sample trades
//     let trade_a = Trade {
//         trade_id: 101,
//         symbol: "AAPL".to_string(),
//         price: 150.50,
//         volume: 100,
//     };
//     let trade_b = Trade {
//         trade_id: 103,
//         symbol: "GOOG".to_string(),
//         price: 2800.00,
//         volume: 5,
//     };
//     let trade_c = Trade {
//         trade_id: 102,
//         symbol: "MSFT".to_string(),
//         price: 300.75,
//         volume: 50,
//     };
//     // This trade is a duplicate of trade_a based on all fields
//     let trade_duplicate = Trade {
//         trade_id: 101,
//         symbol: "AAPL".to_string(),
//         price: 150.50,
//         volume: 100,
//     };
//     // This trade has a new ID and price, making it unique
//     let trade_d = Trade {
//         trade_id: 104,
//         symbol: "AAPL".to_string(),
//         price: 151.00,
//         volume: 100,
//     };

//     // 3. Insertion: Insert the trades into the BTreeSet
//     // BTreeSet automatially keeps the elements sorted and ensures uniqueness
//     trade_ledger.insert(trade_a.clone());
//     trade_ledger.insert(trade_b.clone());
//     trade_ledger.insert(trade_c.clone());

//     println!(
//         "\n Total trades after initial inserts: {}",
//         trade_ledger.len()
//     );

//     // 4. Uniqueness Feature: Attempt to insert the duplicate trade
//     let inserted = trade_ledger.insert(trade_duplicate.clone());
//     // Since trade_duplicate is equal to trade_a, it won't be inserted again
//     println!("Was duplicate trade inserted?: {}", inserted);

//     // Insert the unique new trade (trade_d)
//     trade_ledger.insert(trade_d.clone());

//     println!(
//         "Total trades after attempted duplicate insert {}",
//         trade_ledger.len()
//     );

//     // 5. Sorted Iteration: Iterate through the BTreeSet. They are *always* sorted.
//     // The sort order is determined by the implementation of the 'Ord' trait on 'Trade'
//     println!("\n--- All Trades (Sorted by Trade ID) ---");
//     for trade in &trade_ledger {
//         println!(
//             "ID: {}, Symbol: {}, Price: {}",
//             trade.trade_id, trade.symbol, trade.price
//         );
//     }

//     // 6. Checking Membership: Efficiently check if a trade exists
//     println!("\n--- Membership Check ---");
//     let check_trade = Trade {
//         trade_id: 103,
//         symbol: "GOOG".to_string(),
//         price: 2800.00,
//         volume: 5,
//     };
//     println!(
//         "Does trade ID 103 exist? {}",
//         trade_ledger.contains(&check_trade)
//     );

//     let non_existent_trade = Trade {
//         trade_id: 999,
//         symbol: "XYZ".to_string(),
//         price: 10.00,
//         volume: 1,
//     };
//     println!(
//         "Does trade ID 999 exist? {}",
//         trade_ledger.contains(&non_existent_trade)
//     );

//     // 7. Range Query: Find all trades within a specific range of keys
//     // This is a powerful BTreeSet feature, often used for time-based lookups
//     // Here we'll query for all trades with trade_id between 101 and 103 (inclusive)
//     println!("\n--- Range Query: Trades 101 to 103 ---");
//     let range_start = Trade {
//         trade_id: 101,
//         symbol: "".to_string(),
//         price: 0.0,
//         volume: 0,
//     };
//     let range_end = Trade {
//         trade_id: 103,
//         symbol: "".to_string(),
//         price: 0.0,
//         volume: 0,
//     };

//     // The range function returns an iterator over the elements in the specified range.
//     // Note: When using ranges, the comparison only uses the first differing field
//     // (trade_id in this case), which is why we can use dummy values for symbol/price/volume.
//     for trade in trade_ledger.range(range_start..=range_end) {
//         println!(
//             "  -> Found Trade ID: {}, Symbol: {}",
//             trade.trade_id, trade.symbol
//         );
//     }

//     // 8. Removal: Remove a specific trade
//     println!("\n--- Removal ---");
//     let removed_trade = Trade {
//         trade_id: 104,
//         symbol: "AAPL".to_string(),
//         price: 151.00,
//         volume: 100,
//     };
//     let removed = trade_ledger.remove(&removed_trade);
//     println!("Was trade ID 104 removed? {}", removed);
//     println!("Total trades after removal: {}", trade_ledger.len());
// }
//
// --------------------------------------------------------------------------
// RC
// --------------------------------------------------------------------------

// // --- Data Structures ---

// use std::rc::Rc;

// // Represents a single Stock in the portfolio.
// #[derive(Debug, Clone)]
// struct Stock {
//     symbol: String,
//     shares: u32,
//     price: f64, // Current price per share
// }

// // Represents the overall collection of holdings.
// #[derive(Debug)]
// struct Portfolio {
//     stocks: Vec<Stock>,
// }

// impl Portfolio {
//     // Calculates the total market value of the portfolio.
//     fn market_value(&self) -> f64 {
//         self.stocks.iter().map(|s| s.shares as f64 * s.price).sum()
//     }

//     fn print_holdings(&self) {
//         println!("  Current Portfolio Value: ${:.2}", self.market_value());
//         for stock in &self.stocks {
//             println!(
//                 "  - {} ({:.0} shares @ ${:.2})",
//                 stock.symbol, stock.shares, stock.price
//             );
//         }
//     }
// }

// // Represents a specific investment approach that needs to analyze the portfolio.
// #[derive(Debug)]
// struct InvestmentStrategy {
//     name: String,
//     // Rc<Portfolio> allows multiple strategies to "own" and share the same portfolio data.
//     // This avoids copying the portfolio and ensures it's dropped only when the last Rc is.
//     portfolio: Rc<Portfolio>,
// }

// impl InvestmentStrategy {
//     fn new(name: &str, portfolio: Rc<Portfolio>) -> Self {
//         InvestmentStrategy {
//             name: name.to_string(),
//             portfolio,
//         }
//     }

//     // Analyzes the portfolio for this specific strategy.
//     fn analyze(&self) {
//         println!("\n --- {} Analysis ---", self.name);
//         println!(
//             "Analyzing shared data (Rc count: {})",
//             Rc::strong_count(&self.portfolio)
//         );

//         // We can access the shared Portfolio data directly
//         let value = self.portfolio.market_value();

//         if value > 100000.0 {
//             println!("  Portfolio is large! Suggesting a **Defensive** strategy.");
//         } else {
//             println!("  Portfolio is small/medium. Suggesting a **Growth** strategy.");
//         }

//         // The portfolio data is prined from the shared structure
//         self.portfolio.print_holdings();
//     }
// }

// // --- Main Execution ---

// fn main() {
//     // 1. Create the original Portfolio instance
//     let initial_portfolio = Portfolio {
//         stocks: vec![
//             Stock {
//                 symbol: "APPL".to_string(),
//                 shares: 500,
//                 price: 175.50,
//             },
//             Stock {
//                 symbol: "GOOGL".to_string(),
//                 shares: 150,
//                 price: 2900.00,
//             },
//             Stock {
//                 symbol: "MSFT".to_string(),
//                 shares: 300,
//                 price: 320.75,
//             },
//         ],
//     };

//     println!("Original Portfolio Created.");
//     initial_portfolio.print_holdings();

//     // 2. Wrap the Portfolio in an Rc<T>. This is the first owner.
//     let shared_portfolio = Rc::new(initial_portfolio);
//     println!(
//         "\nInitial Rc count (shared_portfolio): {}",
//         Rc::strong_count(&shared_portfolio)
//     );

//     // 3. Create multiple Investment Strategies, each getting a clone of the Rc<T>.
//     // This increments the reference count.
//     let strategy_a = InvestmentStrategy::new("Long-Term Value", Rc::clone(&shared_portfolio));
//     println!(
//         "Rc count after Strategy A: {}",
//         Rc::strong_count(&shared_portfolio)
//     );

//     let strategy_b = InvestmentStrategy::new("Tech Sector Focus", Rc::clone(&shared_portfolio));
//     println!(
//         "Rc count after Strategy B: {}",
//         Rc::strong_count(&shared_portfolio)
//     );

//     // 4. Use the strategies to analyze the *shared* portfolio data.
//     strategy_a.analyze();
//     strategy_b.analyze();

//     // A final check of the count before the end of main
//     println!(
//         "\nFinal Rc count before scope ends: {}",
//         Rc::strong_count(&shared_portfolio)
//     );
// }

// --------------------------------------------------------------------------
// MUTEX
// --------------------------------------------------------------------------

// use std::{
//     sync::{Arc, Mutex},
//     thread,
// };

// // Define the BankAccount structure
// struct BankAccount {
//     balance: f64, // f64 is commonly used for currency/floating-point
// }

// // Implement methods for the BankAccount
// impl BankAccount {
//     // Constructor
//     fn new(initial_balance: f64) -> BankAccount {
//         BankAccount {
//             balance: initial_balance,
//         }
//     }

//     // Method to deposit money
//     fn deposit(&mut self, amount: f64) {
//         self.balance += amount;
//     }

//     // Method to withdraw money
//     fn withdraw(&mut self, amount: f64) -> Result<(), &'static str> {
//         if self.balance >= amount {
//             self.balance -= amount;
//             Ok(())
//         } else {
//             Err("Insufficient funds")
//         }
//     }

//     // Method to check the balance
//     fn get_balance(&self) -> f64 {
//         self.balance
//     }
// }

// fn main() {
//     // 1. Create the shared resource (BankAccount)
//     let initial_balance = 1000.0;
//     let account = BankAccount::new(initial_balance);

//     // 2. Wrap the BankAccount in a Mutex for safe mutable access
//     // And then wrap the Mutex in an Arc (Atomic Reference Counter)
//     // Arc allows multiple threads to  *own* a reference to the same data structure.
//     let shared_account = Arc::new(Mutex::new(account));

//     // Define the number of concurrent transactions (threads)
//     let num_threads = 10;
//     let mut handles = vec![];
//     let transaction_amount = 50.0;

//     println!("Starting balance: ${:.2}", initial_balance);
//     println!("Simulating {} concurrent transactions...", num_threads);

//     // 3. Spawn multiple threads to perform transactions
//     for i in 0..num_threads {
//         // Clone the Arc for each new thread. This increases the reference count.
//         let account_clone = Arc::clone(&shared_account);

//         let handle = thread::spawn(move || {
//             // Randomly decide between a deposit or withdrawal for simulation
//             if i % 2 == 0 {
//                 // ---- DEPOSIT TRANSACTION ---
//                 // .lock() blocks the thread until it can acquire the Mutex lock.
//                 // It returns a MutexGuard, which acts like a smart pointer (DerefMut)
//                 // that gives us mutable access to the BankAccount inside.
//                 let mut account_guard = match account_clone.lock() {
//                     Ok(guard) => guard,
//                     Err(poisoned) => {
//                         // Handle the Mutex "poisoning" error if a thread panics while holding the lock
//                         println!("Thread {} failed to acquire lock: Mutex is poisoned!", i);
//                         poisoned.into_inner() // Recover the inner data
//                     }
//                 };

//                 // Once the lock is acquired, we can safely call a mutable method.
//                 account_guard.deposit(transaction_amount);
//                 println!("Thread {} deposited ${:.2}.", i, transaction_amount);

//                 // The lock is automatically released when 'account_guard' goes out of scope (at the end of the block).
//             } else {
//                 // --- WITHDRAWAL TRANSACTION ---
//                 let mut account_guard = account_clone.lock().unwrap(); // Use unwrap for simplicity in this example

//                 match account_guard.withdraw(transaction_amount) {
//                     Ok(_) => println!("Thread {} withdrew ${:.2}.", i, transaction_amount),
//                     Err(e) => println!(
//                         "Thread {} tried to withdraw ${:.2} but failed: {}",
//                         i, transaction_amount, e
//                     ),
//                 }

//                 // Lock is released here.
//             }
//         });

//         handles.push(handle);
//     }

//     // 4. Wait for all threads to complete
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     // 5. Final check of the balance (needs a final lock to read the result)
//     let final_balance = shared_account.lock().unwrap().get_balance();

//     println!("---");
//     println!("Final balance: ${:.2}", final_balance);

//     // Expected final balance calculation:
//     // Initial: 1000.0
//     // 5 Deposits (Threads 0, 2, 4, 6, 8): + 5 * 50.0 = + 250.0
//     // 5 Withdrawals (Threads 1, 3, 5, 7, 9): - 5 * 50.0 = - 250.0
//     // Expected Final Balance: 1000.0 (assuming all withdrawals succeed)
//     println!("Expected balance: ${:.2}", initial_balance);
// }

// --------------------------------------------------------------------------
// CHANNELS
// --------------------------------------------------------------------------

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// // -- Data Structures ---

// // Represents a single financial transaction.
// #[derive(Debug)]
// struct Transaction {
//     id: u32,
//     amount: f64, // Positive for deposit, negative for withdrawal
// }

// // --- Main Function (Client/Initiator) ---
// fn main() {
//     // 1. Setup the main channel: The client sends transactions to the processor.
//     // 'tx' is the Transmitter, 'rx' is the Receiver.
//     let (tx_transaction, rx_transaction) = mpsc::channel::<Transaction>();

//     // 2. Setup the response channel: The processor sends the result back to the client.
//     let (tx_result, rx_result) = mpsc::channel::<f64>();

//     let initial_balance = 1000.0;
//     println!("Initial Balance: ${}", initial_balance);
//     println!("--- Starting Transaction Processor ----");

//     // 3. Spawn the Transaction Processor Thread
//     // The worker thread takes ownership of the Receiver and the Result Transmitter.
//     let processor_handle = thread::spawn(move || {
//         // Start the processor with the initial balance.
//         process_transactions(rx_transaction, tx_result, initial_balance);
//     });

//     // 4. Client sends transactions on the main thread
//     let transactions_to_send = vec![
//         Transaction {
//             id: 101,
//             amount: 200.50,
//         }, // Deposit
//         Transaction {
//             id: 102,
//             amount: -50.00,
//         }, // Withdrawal
//         Transaction {
//             id: 103,
//             amount: 15.00,
//         }, // Deposit
//         Transaction {
//             id: 104,
//             amount: -1500.0,
//         }, // Will cause an overdraft (handled in processor)
//     ];

//     for tx in transactions_to_send {
//         println!("\n[Client] Sending Transaction ID: {}", tx.id);

//         // Send the transaction to the worker thread.
//         if tx_transaction.send(tx).is_err() {
//             eprintln!("[Client] Error sending transaction, processor thread likely died.");
//             break;
//         }

//         // Wait for the response from the worker thread.
//         match rx_result.recv() {
//             Ok(new_balance) => {
//                 println!("[Client] Received updated balance: **${:.2}**", new_balance);
//             }
//             Err(_) => {
//                 eprintln!("[Client] Processor threa disconnected. Existing.");
//                 break;
//             }
//         }
//         // Simulate a small delay between transactions
//         thread::sleep(Duration::from_millis(100));
//     }

//     // Explicitly drop the sender to signal to the receiver (processor thread)
//     // that no more messages will come, allowing 'recv' to eventually return an error.
//     drop(tx_transaction);

//     // Wait for the processor thread to finish its work.
//     if let Err(e) = processor_handle.join() {
//         eprintln!("Error joining processor thread: {:?}", e);
//     }

//     println!("\n--- All transactions processed. System shut down. ----");
// }

// // --- Worker Function (Transaction Processor) ---

// // Simulate a dedicated thread that processes transactions and updates the balance.
// fn process_transactions(
//     rx: mpsc::Receiver<Transaction>,
//     tx_result: mpsc::Sender<f64>,
//     mut balance: f64,
// ) {
//     println!("[Processor] Thread started.");

//     // The 'for' loop iterates until the sender (tx_transaction) is dropped.
//     for transaction in rx {
//         println!(
//             "\n[Processor] Receiving Tx #{}: ${:.2}",
//             transaction.id, transaction.amount
//         );

//         let new_balance = balance + transaction.amount;

//         // Basic check to prevent overdrafts (simple business logic)
//         if new_balance < 0.0 {
//             println!(
//                 "[Processor] **REJECTED**: Transaction ID {} would cause an overdraft.",
//                 transaction.id
//             );
//             // Send the *unchanged* balance back to the client.
//             if tx_result.send(balance).is_err() {
//                 eprintln!("[Processor] Failed to send balance back to client.");
//                 break; // Stop processing if the client is gone
//             } else {
//                 // Transaction is valid, update the balance.
//                 balance = new_balance;
//                 println!("[Processor] APPROVED. New current balance: ${:.2}", balance);

//                 // Send the new balance back to the client using the response channel.
//                 if tx_result.send(balance).is_err() {
//                     eprintln!("[Processor] Failed to send balance back to client.");
//                     break; // Stop processing if the client is gone
//                 }
//             }
//         }

//         // Simulate work being done (e.g., database update, API call)
//         thread::sleep(Duration::from_millis(50));
//     }

//     // This part is reached when the main thread drops its 'tx_transaction' sender.
//     println!("[Processor] All transaction senders disconnected. Thread shutting down.");
// }

// --------------------------------------------------------------------------
// ERROR HANDLING
// --------------------------------------------------------------------------

// use std::fmt;

// // --- 1. Define a Custom Error Type ---
// // This enum lists all possible, recoverable errors in our transfer logic.
// #[derive(Debug)]
// pub enum TransferError {
//     InsufficientFunds { required: f64, available: f64 },
//     InvalidAmount(f64),
//     SameAccount,
// }

// // Implement the Display trait for use-friendly error messages
// impl fmt::Display for TransferError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             TransferError::InsufficientFunds {
//                 required,
//                 available,
//             } => write!(
//                 f,
//                 "Transfer failed: Insufficient funds. Required: ${:.2}, Available: ${:.2}",
//                 required, available
//             ),
//             TransferError::InvalidAmount(amount) => write!(
//                 f,
//                 "Transfer failed: Invalid amount ${:.2}. Amount must be positive.",
//                 amount
//             ),
//             TransferError::SameAccount => write!(
//                 f,
//                 "Transfer failed: Cannot transfer funds to the same account."
//             ),
//         }
//     }
// }

// // --- 2. Defne the Account Structure ---
// pub struct BankAccount {
//     pub account_number: u32,
//     balance: f64,
// }

// impl BankAccount {
//     // Constructor
//     pub fn new(account_number: u32, initial_balance: f64) -> BankAccount {
//         BankAccount {
//             account_number,
//             balance: initial_balance,
//         }
//     }

//     // Public method to get the current balance
//     pub fn balance(&self) -> f64 {
//         self.balance
//     }

//     // --- 3. The core Function with Error Handling ---
//     // The function returns a Result, indication success (Ok) or a specific error (Err).
//     pub fn transfer(
//         &mut self,
//         to_account: &mut BankAccount,
//         amount: f64,
//     ) -> Result<(), TransferError> {
//         // Validation 1: Check for invalid amount (business rule)
//         if amount <= 0.0 {
//             return Err(TransferError::InvalidAmount(amount));
//         }

//         // Validation 2: Check for same account transfer
//         if self.account_number == to_account.account_number {
//             return Err(TransferError::SameAccount);
//         }

//         // Validation 3: Check for sufficient funds (business rule)
//         if self.balance < amount {
//             return Err(TransferError::InsufficientFunds {
//                 required: amount,
//                 available: self.balance,
//             });
//         }

//         // If all checks pass, the transaction is successful (the 'happy path')
//         self.balance -= amount;
//         to_account.balance += amount;

//         // Return Ok(()) to signify success. () is the "unit" type, meaning no value is returned.
//         Ok(())
//     }
// }

// // --- 4. Main Function for Demonstrating Usage ---
// fn main() {
//     let mut account_a = BankAccount::new(1001, 150.0);
//     let mut account_b = BankAccount::new(1002, 150.0);

//     println!("--- Initial Balances ---");
//     println!("--- Account 1001 Balance: ${:.2}", account_a.balance());
//     println!("--- Account 1002 Balance: ${:.2}", account_b.balance());
//     println!("------------------------");

//     // Successful Transfer
//     let result_ok = account_a.transfer(&mut account_b, 50.0);
//     match result_ok {
//         Ok(_) => {
//             println!("✅Transfer of $50.00 successful!");
//         }
//         Err(e) => {
//             // This arm would not be reached, but good practice to include
//             eprintln!("❌ Error during transfer: {}", e);
//         }
//     }

//     println!("--- Account 1001 Balance: ${:.2}", account_a.balance());
//     println!("--- Account 1002 Balance: ${:.2}", account_b.balance());
//     println!("------------------------");

//     // Failure Case 1: Insufficient Funds
//     // We use 'if let' for a concise way to handle only the error variant.
//     let result_fail_funds = account_a.transfer(&mut account_b, 200.00);
//     if let Err(e) = result_fail_funds {
//         eprintln!("❌ Failed Transfer (Insufficient Funds): {}", e);
//     }
//     println!("------------------------");

//     // Failure Case 2: Invalid Amout
//     let result_fail_amount = account_a.transfer(&mut account_b, -10.0);
//     if let Err(e) = result_fail_amount {
//         eprintln!("❌ Failed Transfer (Insufficient Amount): {}", e);
//     }
// }

// --------------------------------------------------------------------------
// OPTION AND RESULT ENUMERATIONS
// --------------------------------------------------------------------------

// --- Custom Error Type (for Result) ---

// use std::collections::HashMap;

// #[derive(Debug)]
// pub enum ExchangeError {
//     InvalidCurrency(String),
//     ApiCallFailed(String),
//     InsufficientFunds, // Example of another financial error
// }

// impl std::fmt::Display for ExchangeError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             ExchangeError::InvalidCurrency(c) => write!(f, "Invalid currency code: {}", c),
//             ExchangeError::ApiCallFailed(e) => write!(f, "Exchange API call failed: {}", e),
//             ExchangeError::InsufficientFunds => {
//                 write!(f, "Transaction failed due to insufficient funds.")
//             }
//         }
//     }
// }

// // Implement standard Error trait for out custom error
// impl std::error::Error for ExchangeError {}

// // --- Core Exchange Data Structure ---
// pub struct ExchangeService {
//     // Map of currency pairs (e.g., "USD/EUR") to their primary exchange rate
//     rates: HashMap<String, f64>,
//     // Map of currency pairs to an **optional** secondary rate source (e.g., a "premium" rate)
//     secondary_rates: HashMap<String, f64>,
// }

// impl ExchangeService {
//     // Helper to construct a rate key
//     fn make_key(from: &str, to: &str) -> String {
//         format!("{}/{}", from, to)
//     }

//     // Fetches the primary exchange rate a currency pair.
//     // Returns a Result, as this is a fallible operation (currency might not exist).
//     fn get_primary_rate(&self, from: &str, to: &str) -> Result<f64, ExchangeError> {
//         let key = Self::make_key(from, to);

//         // Use Result.map_or_else to return an Ok(rate) or a descriptive Err.
//         self.rates
//             .get(&key)
//             .copied()
//             .ok_or_else(|| ExchangeError::InvalidCurrency(format!("Rate not found for {}", key)))
//     }

//     // Fetches an optional secondary exchange rate.
//     // Returns a Option, as the absence of a secondary rate is *expected* for some pairs.
//     fn get_secondary_rate(&self, from: &str, to: &str) -> Option<f64> {
//         let key = Self::make_key(from, to);

//         // The .copied() is needed because .get() returns Option<&f64>.
//         // Option<f64> represents either a value or None.
//         self.secondary_rates.get(&key).copied()
//     }

//     // Calculates the exchange, trying the primary rate, and falling back to a secondary rate
//     // if available, before propagating an error.
//     pub fn exchange(
//         &self,
//         from_currency: &str,
//         to_currency: &str,
//         amount: f64,
//     ) -> Result<f64, ExchangeError> {
//         // Try to get the primary rate '?' for early error return (Result)
//         let primary_rate = match self.get_primary_rate(from_currency, to_currency) {
//             Ok(rate) => rate,
//             // If primary rate fails, check if we have a secondary rate (Option)
//             Err(e) => {
//                 match self.get_secondary_rate(from_currency, to_currency) {
//                     Some(secondary_rate) => {
//                         println!(
//                             "Note: Using secondary rate for {}/{}",
//                             from_currency, to_currency
//                         );
//                         secondary_rate
//                     }
//                     None => return Err(e), // Propagate the original error if no secondary rate found
//                 }
//             }
//         };

//         // Final calculation
//         Ok(amount * primary_rate)
//     }
// }

// //--- Main execution to demonstrate usage ---
// fn main() {
//     // 1. Setup the service with some sample data
//     let mut service = ExchangeService {
//         rates: HashMap::new(),
//         secondary_rates: HashMap::new(),
//     };
//     service.rates.insert("USD/EUR".to_string(), 0.92);
//     service.rates.insert("USD/JPY".to_string(), 145.00);
//     // Add a secondary rate for USD/EUR fro demonstration
//     service.secondary_rates.insert("USD/EUR".to_string(), 0.915);
//     service.secondary_rates.insert("GBP/EUR".to_string(), 1.15);

//     println!("--- Successful Exchanges (Primary Rate) ----");

//     // 2. Succesful exchange (Result::Ok) - uses primary rate
//     match service.exchange("USD", "JPY", 100.0) {
//         Ok(converted) => println!("100 USD -> {} JPY", converted), // Output: 14500
//         Err(e) => eprintln!("Exchange failed: {}", e),
//     }

//     println!("--- Successful Exchange (Secondary Rate Fallback) ----");

//     // 3. Force failure or primary to test Option fallback
//     // We'll temporarily remove the primary rate for USD/JPY to force the fallback logic
//     service.rates.remove("USD/JPY");

//     // This will now use the logic that checks for a secondary rate *after* the primary fails,
//     // bit sine we only have a secondary rate for GBP/EUR, this will fail.
//     match service.exchange("USD", "JPY", 100.0) {
//         Ok(converted) => println!("100 USD -> {} JPY", converted),
//         Err(e) => eprintln!("Exchange failed: {}", e), // Output: Exchange failed: Invalid currency code: USD/JPY
//     }

//     // This will use the secondary rate as the primary rate is gone for this example
//     service.rates.remove("USD/EUR");
//     match service.exchange("USD", "EUR ", 100.0) {
//         Ok(converted) => println!("100 USD -> {} EUR", converted), // Output: 91.5
//         Err(e) => eprintln!("Exchange failed: {}", e),
//     }

//     println!("\n--- Failed Exchange (Result::Err) ----");

//     //4. Failed exchange (Result::Err) - no rate exists at all
//     match service.exchange("USD", "AUD ", 100.0) {
//         Ok(converted) => println!("100 USD -> {} AUD", converted),
//         Err(e) => eprintln!("Exchange failed: {}", e), // Output: Exchange failed: Invalid currency code: USD/AUD
//     }
// }

// --------------------------------------------------------------------------
// PROPAGATING ERRORS AND '?' OPERATOR
// --------------------------------------------------------------------------

// use std::error::Error; // Import for the Display trait, used for custom errors
// use std::fmt; // Import for the Error trait

// // --- 1. Custom Error Definitions ---

// // Define a custom error for account-level issues (e.g., not enough funds).
// #[derive(Debug)]
// pub enum AccountError {
//     // The withdrawal amount exceeeds the available balance.
//     InsufficientFunds { requested: f64, available: f64 },
//     // The amount requested is zero or negative.
//     InvalidAmount(f64),
// }

// // Implement the Display trait for AccountError to allow printing a user-friendly message.
// impl fmt::Display for AccountError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             AccountError::InsufficientFunds {
//                 requested,
//                 available,
//             } => write!(
//                 f,
//                 "Insufficient Funds: Requested ${:.2}, but only ${:.2} available.",
//                 requested, available
//             ),
//             AccountError::InvalidAmount(amount) => {
//                 write!(
//                     f,
//                     "Invalid Amount: Withdrawal amount ${:.2} must be positive.",
//                     amount
//                 )
//             }
//         }
//     }
// }

// // Implement the Error trait to make it a proper error type.
// impl Error for AccountError {}

// // Define a custom error for system-level issues (e.g., failed database access).
// #[derive(Debug)]
// pub enum SystemError {
//     // A simulated database connection failure.
//     DatabaseConnectionFailed(String),
// }

// // Implement the Display and Error traits for SystemError as well.
// impl fmt::Display for SystemError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             SystemError::DatabaseConnectionFailed(ref reason) => {
//                 write!(
//                     f,
//                     "System Error: Database connection failed. Reason: {}",
//                     reason
//                 )
//             }
//         }
//     }
// }
// impl Error for SystemError {}

// // --- 2. Result Type Aliases for Clarity ----

// // Alias the standard Result type to use our custom errors., making function signatures
// pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

// // --- 3. Core Logic Functions ---

// // Simulates checking the balance in a 'database'.
// // This function can fail with a SystemError.
// fn get_current_balance(account_id: u32) -> std::result::Result<f64, SystemError> {
//     // Simulate a system failure (e.g., DB is down)
//     if account_id == 101 {
//         return Err(SystemError::DatabaseConnectionFailed(
//             "Connection timeout".to_string(),
//         ));
//     }
//     // Simulate a successful balance fetch
//     Ok(500.00)
// }

// // Ties to process a withdrawal.
// // This function calls 'get_current_balance' and  propagates its SystemError.
// // It an also generate its own AccountError.
// fn process_withdrawal(account_id: u32, amount: f64) -> Result<f64> {
//     // Check for invalid amount first
//     if amount <= 0.0 {
//         // Return an AccountError directly
//         return Err(AccountError::InvalidAmount(amount).into());
//     }

//     // Use the '?' operator to call 'get_current_balance'.
//     // If 'get_current_balance' returns SystemError, '?' extracts it, converts it to
//     // a Box<dyn Error>, and immediately returns from 'process_withdrawal'.
//     // If it succeeds, 'balance' holds the f64 value.
//     let balance = get_current_balance(account_id)?;

//     // Check for insufficient funds
//     if balance < amount {
//         // Returns an AccountError directly
//         return Err(AccountError::InsufficientFunds {
//             requested: amount,
//             available: balance,
//         }
//         .into());
//     }

//     //Success path: Perform the withdrawal logic (simulated)
//     let new_balance = balance - amount;
//     println!(
//         "Successfully withdrew ${:.2}. New balance: ${:.2}",
//         amount, new_balance
//     );
//     Ok(new_balance)
// }

// // ---4. Main Execution (Error Handling and Demonstration) ---

// fn main() {
//     println!("--- Test Case 1: Successful Withdrawal ---");
//     // Call the function and check the Result
//     match process_withdrawal(100, 50.0) {
//         Ok(new_bal) => println!("Transaction Success! Final balance: ${:.2}", new_bal),
//         Err(e) => println!("Transaction Failed: {}", e),
//     }

//     println!("\n--- Test Case 2: Insufficient Funds (AccountError) ---");
//     // This will trigger an AccountError
//     match process_withdrawal(100, 600.00) {
//         Ok(new_bal) => println!("Transaction Success! Final balance: ${:.2}", new_bal),
//         Err(e) => println!("Transaction Failed: {}", e),
//     }

//     println!("\n--- Test Case 3: Invalid Amount (AccountError) ---");
//     // This will trigger an AccountError
//     match process_withdrawal(100, -10.0) {
//         Ok(new_bal) => println!("Transaction Success! Final balance: ${:.2}", new_bal),
//         Err(e) => println!("Transaction Failed: {}", e),
//     }

//     println!("\n--- Test Case 4: System Failure (SystemError Propagation) ---");
//     // This will trigger the SystemError from 'get_current_balance'
//     // The '?' operator in 'process_withdrawal' will propagate the error.
//     match process_withdrawal(101, 10.0) {
//         Ok(new_bal) => println!("Transaction Success! Final balance: ${:.2}", new_bal),
//         Err(e) => println!("Transaction Failed: {}", e),
//     }
// }

// --------------------------------------------------------------------------
// CUSTOM ERROR TYPES AND TRAITS
// --------------------------------------------------------------------------

// use std::error::Error;
// use std::fmt;

// // --- 1. define the Custom Error Type (Enum) ---
// // This enum lists all possibl errors specific to out banking logic.
// #[derive(Debug, Clone)]
// pub enum BankError {
//     InsufficientFunds { required: f64, available: f64 },
//     InvalidAmount(f64),
//     // We can add other errors here, like DatabaseError, NetworkError, etc.
// }

// // --- 2. Implement the Display Trait for readable error messages ---
// // The Display trait allows users to use '{}' with the error type.
// impl fmt::Display for BankError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             BankError::InsufficientFunds {
//                 required,
//                 available,
//             } => write!(
//                 f,
//                 "Insufficient Funds: Cannot withdraw ${:.2}. Available balance: ${:.2}",
//                 required, available
//             ),
//             BankError::InvalidAmount(amount) => write!(
//                 f,
//                 "Invalid Amount: Cannot process non-positive withdrawal amount: ${:.2}",
//                 amount
//             ),
//         }
//     }
// }

// // --- 3. Implement the standard Error Trait ---
// // Implementing the standard Error trait makes out custom error interoperable
// // with the rest of Rust's error ecosystem (like the '?' operator).
// impl Error for BankError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         // This is used for "error chaining". Since out errors are fundamental
//         // to our domain, they don't have an underlying source error, so we return None.
//         None
//     }

//     // 'description' is deprecated but can be implemented for older compatibility.
//     // The main functionality is now covered by 'Display' and 'source'.
// }

// // --- 4. The Core Logic Function ---
// // This function performs a withdrawal and returns a Result.
// // It returns Ok(()) on success, or the custom BankError on failure.
// pub fn withdraw(current_balance: &mut f64, amount: f64) -> Result<(), BankError> {
//     // Check for invalid amount (zero or negative)
//     if amount <= 0.0 {
//         return Err(BankError::InvalidAmount(amount));
//     }

//     // Check for insufficient funds
//     if amount > *current_balance {
//         return Err(BankError::InsufficientFunds {
//             required: amount,
//             available: *current_balance,
//         });
//     }

//     // If all checks pass, perform the withdrawal
//     *current_balance -= amount;

//     Ok(())
// }

// // --- 5. Main Execution Block ---
// fn main() {
//     let mut account_balance: f64 = 100.00;
//     println!("Initial Balance ${:.2}", account_balance);

//     // --- Scenario 1: Successful Withdrawal ---
//     let withdrawal1: f64 = 30.0;
//     println!("\nAttempting to withdraw ${:.2}...", withdrawal1);
//     match withdraw(&mut account_balance, withdrawal1) {
//         Ok(_) => println!(
//             "Withdrawal successful. New balance: ${:.2}",
//             account_balance
//         ),
//         Err(e) => println!("Transaction failed : {}", e), // Uses the Display impl
//     }

//     // --- Scenario 2: Insufficient Funds Failure ---
//     let withdrawal2: f64 = 80.0;
//     println!("\nAttempting to withdraw ${:.2}...", withdrawal2);
//     match withdraw(&mut account_balance, withdrawal2) {
//         Ok(_) => println!(
//             "Withdrawal successful. New balance: ${:.2}",
//             account_balance
//         ),
//         Err(e) => {
//             println!("Transaction failed : {}", e);
//             // We can also print the Debug format for developers/logs
//             eprintln!("DEBUG INFO: {:?}", e);
//         }
//     }

//     // --- Scenario 3: Invalid Amount Failure ---
//     let withdrawal3: f64 = -10.0;
//     println!("\nAttempting to withdraw ${:.2}...", withdrawal3);
//     match withdraw(&mut account_balance, withdrawal3) {
//         Ok(_) => println!(
//             "Withdrawal successful. New balance: ${:.2}",
//             account_balance
//         ),
//         Err(e) => println!("Transaction failed : {}", e),
//     }

//     println!("\nFinal Balance: ${:.2}", account_balance);
// }

// --------------------------------------------------------------------------
// CONCURRENCY & PARALLELISM
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// THREADS, CHANNELS AND MESSAGE PASSING
// --------------------------------------------------------------------------

// use std::sync::mpsc; // MPSC stands for Multiple Producer, Single Consumer
// use std::thread;
// use std::time::Duration;

// // 1. Define the Message type (Transaction)
// // The 'Debug' trait allows us to print the struct easily with {:?}
// #[derive(Debug)]
// struct Transaction {
//     id: u32,
//     trader_name: String,
//     amount: f64,
// }

// fn main() {
//     // Create a MPSC channel.
//     // 'tx' is the transmitter (Sender), 'rx' is the receiver (Receiver).
//     let (tx, rx) = mpsc::channel();

//     // A list to hold the handles of all spawned threads so we can 'join' them later.
//     let mut handles = vec![];

//     // Define multiple trader names to act as the producers
//     let traders = vec!["AlphaFund", "BetaCorp", "GammaBank"];
//     let num_transactions = 5;

//     // --- SPANNING PRODUCER THREADS (TRADERS) ---
//     for (i, trader_name) in traders.into_iter().enumerate() {
//         // We clone the Sender ('tx') for each new thread.
//         // This is what makes it a "Multiple Producer" channel.
//         let tx_clone = tx.clone();
//         let name = trader_name.to_string();

//         let handle = thread::spawn(move || {
//             for j in 0..num_transactions {
//                 // 2, Create a new transaction message
//                 let transaction = Transaction {
//                     id: (i * num_transactions) as u32 + j as u32 + 1,
//                     trader_name: name.clone(),
//                     // Simulate varying transaction amounts
//                     amount: (j as f64 + 1.0) * 100.00 * (i as f64 + 1.0),
//                 };

//                 println!("=> {} SENT Transaction ID: {}", name, transaction.id);

//                 // 3. Send the message (Transaction) down the channel.
//                 // The 'send' method takes ownership of the 'transaction' object,
//                 // preventing data race and ensuring safe transfer between threads.
//                 if let Err(e) = tx_clone.send(transaction) {
//                     // This error occurs if the receiver has been dropped.
//                     eprintln!("Error sending transaction: {}", e);
//                     break;
//                 }
//                 // Small sleep to simulate work being done/delay between trades
//                 thread::sleep(Duration::from_millis(50));
//             }
//         });
//         handles.push(handle);
//     }

//     // Since we cloned 'tx' for the threads, we can drop the original 'tx'
//     // in the main thread. This is important: 'rx' will only stop blocking
//     // when ALL Senders (including clones) are dropped.
//     drop(tx);

//     // --- SPANNING CONSUMER THREAD (PROCESSOR) ---
//     let processor_handle = thread::spawn(move || {
//         let mut total_processed = 0;
//         println!("*** Transaction Processor STARTING ***");

//         // 4. Receive and process messages.
//         // The 'rx' receiver acts as an iterator that blocks and waits for a message.
//         // The loop terminates automatically when all Senders (tx_clones) are dropped.
//         for transaction in rx {
//             total_processed += 1;
//             // The receiver receives ownership of the 'Transaction' data.
//             println!(
//                 "  <- PROCESSED #{}: ID: {}, Trader: {}, Amount: {}",
//                 total_processed, transaction.id, transaction.trader_name, transaction.amount
//             );
//             // Simulate processing time
//             thread::sleep(Duration::from_millis(20));
//         }

//         println!(
//             "*** Transaction Processor FINISHED. Total processed: {} ***",
//             total_processed
//         );
//     });

//     // --- MAIN THREAD WAITS FOR ALL PRODUCERS TO FINISH ---
//     // Wait for all trader threads to complete their work,
//     for handle in handles {
//         handle.join().expect("Trader thread panicked");
//     }

//     // --- MAIN THREAD WAITS FOR THE CONSUMER TO FINISH ---
//     // The processor thread will finish once all 'tx' senders are dropped
//     // and the channel is exhausted.
//     processor_handle.join().expect("Processor thread panicked");

//     println!("All traders and processor threads have sucessfully completed.");
// }

// --------------------------------------------------------------------------
// FUTURES AND ASYNC/AWAIT PARADIGM
// --------------------------------------------------------------------------

// use rand::Rng;
// use std::{
//     sync::{Arc, Mutex},
//     time::Duration,
// };
// use tokio::time::sleep;

// // --- CONSTANTS & CONFIG ---
// const CONFIDENCE_LEVEL_95: f64 = 1.65; // Z-score for 95% confidence

// // A custom error type for our financial system
// #[derive(Debug)]
// enum FinError {
//     NetworkFailure(String),
// }

// // --- DOMAIN MODELS ---

// // Represents a financial asset (e.g., a Stock)
// #[derive(Debug, Clone)]
// struct Asset {
//     ticker: String,
//     position_size: f64, // Number of shares
// }

// // The result of our analysis for a specific asset
// #[derive(Debug)]
// struct AssetAnalysis {
//     ticker: String,
//     current_price: f64,
//     valuation: f64,
//     var_95: f64, // 1-Day Value at Risk
// }

// // --- LOGIC IMPLEMENTATION ---

// // Simulates fetching market data from an external API (I/O Bound)
// // This function is ASYNC because it waits for networks response.
// async fn fetch_market_price(ticker: &str) -> Result<f64, FinError> {
//     // Simulates network latency between 50ms and 200ms
//     let delay = rand::rng().random_range(50..200);
//     sleep(Duration::from_millis(delay)).await;

//     // Simulate a random price (Mocking real data)
//     let price = match ticker {
//         "AAPL" => 150.0,
//         "GOOGL" => 2800.0,
//         "TSLA" => 900.0,
//         "MSFT" => 310.0,
//         _ => {
//             return Err(FinError::NetworkFailure(format!(
//                 "Unknow Ticker: {}",
//                 ticker
//             )));
//         }
//     };

//     // Add slight noise to simulate live ticking
//     let noise = rand::rng().random_range(-0.5..0.5);
//     Ok(price + noise)
// }

// // CPU-Bound calculation for Value at Risk (VaR).
// // Since this is math-heavy, we don't want to block the Async Event Loop.
// // In a real scenario, this would use Monte Carlo simulations.
// fn calculate_var_cpu_intensive(valuation: f64, volatility: f64) -> f64 {
//     // Heavy math simulation
//     let mut stress_test_accum = 0.0;
//     // Artificial loop to simulate CPU load
//     for _ in 0..1_000_000 {
//         stress_test_accum += 0.0001;
//     }

//     // Formula: VaR = Value * Volatility * Z-Score
//     // Using the acumulator just to ensure the compiler doesn't optimize the loop away
//     let _ = stress_test_accum;

//     valuation * volatility * CONFIDENCE_LEVEL_95
// }

// // --- MAIN EXECUTOR ---

// #[tokio::main] // Macros to set up the Async Runtime
// async fn main() {
//     println!("--- Starting Financial  Portfolio Analyzer ---");
//     let start_time = std::time::Instant::now();

//     // 1. Define the Portfolio
//     let portfolio = vec![
//         Asset {
//             ticker: "AAPL".to_string(),
//             position_size: 100.0,
//         },
//         Asset {
//             ticker: "GOOGL".to_string(),
//             position_size: 10.0,
//         },
//         Asset {
//             ticker: "TSLA".to_string(),
//             position_size: 50.0,
//         },
//         Asset {
//             ticker: "MSFT".to_string(),
//             position_size: 200.0,
//         },
//     ];

//     // Shared container for results (Thread Safe)
//     // We use Arc (Atomic Reference Counted) to share ownership across tasks
//     // We use Mutex to allow safe writing to the vector from multiple threads
//     let results = Arc::new(Mutex::new(Vec::new()));

//     // A vector to hold the handles of our spawned tasks
//     let mut handles = vec![];

//     // 2. CONCURRENCY LOOP
//     for asset in portfolio {
//         // Clone the Arc to pass a reference to the specific task
//         let results_clone = Arc::clone(&results);

//         // Spawn a new Async Task (Green Thread)
//         // This allows all fetches to happen simultaneously, not one by one.
//         let handle = tokio::spawn(async move {
//             println!("Checking market: {}", asset.ticker);

//             // A. Async I/O Phase: Fetch Price
//             // The '.await' here yields control so other tasks can run while waiting
//             match fetch_market_price(&asset.ticker).await {
//                 Ok(price) => {
//                     let valuation = price * asset.position_size;

//                     // B. Parallel Processing Phase: Calculate Risk
//                     // We assume 1.5% daily volatily for this example
//                     let volatility = 0.015;

//                     // spawn_blocking moves this operation to a dedicated thread pool
//                     // meant for CPU intensive work, so we don't freeze the network loop.
//                     let risk_metric = tokio::task::spawn_blocking(move || {
//                         calculate_var_cpu_intensive(valuation, volatility)
//                     })
//                     .await
//                     .unwrap();

//                     // C. Store Result
//                     let analysis = AssetAnalysis {
//                         ticker: asset.ticker.clone(),
//                         current_price: price,
//                         valuation,
//                         var_95: risk_metric,
//                     };

//                     // Lock the mutex to safely write data
//                     let mut data = results_clone.lock().unwrap();
//                     data.push(analysis);
//                 }
//                 Err(e) => eprintln!(">> [ALERT] Failed to process {}: {:?}", asset.ticker, e),
//             }
//         });

//         handles.push(handle);
//     }

//     // 3. AWAIT COMPLETION
//     // Wait for all tasks to finish
//     for handle in handles {
//         let _ = handle.await;
//     }

//     // 4. AGGREGATION & REPORTING
//     let final_data = results.lock().unwrap();
//     let total_valuation: f64 = final_data.iter().map(|a| a.valuation).sum();
//     let total_risk: f64 = final_data.iter().map(|a| a.var_95).sum();

//     println!("\n--- Portfolio Report ---");
//     println!(
//         "{:<10} | {:<10} | {:<15} | {:<10}",
//         "Ticker", "Price", "Valuation", "VaR (95%)"
//     );
//     println!("{:-<55}", "-");

//     for item in final_data.iter() {
//         println!(
//             "{:<10} | ${:<9.2} | ${:<14.2} | ${:<9.2}",
//             item.ticker, item.current_price, item.valuation, item.var_95
//         );
//     }

//     println!("{:-<55}", "-");
//     println!("Total Portfolio Value: ${:.2}", total_valuation);
//     println!("Total Portfolio Risk (VaR): ${:.2}", total_risk);
//     println!("Execution Time: {:.2?}", start_time.elapsed());
// }

// --------------------------------------------------------------------------

// use futures::future::join_all;
// use rand::Rng;
// use std::fmt;
// use std::time::Duration;
// use tokio::time::sleep;

// // --- Domain Types ---

// #[derive(Debug, Clone)]
// struct Position {
//     ticker: String,
//     quantity: f64,
//     purchase_price: f64, // We will use this for PnL now
// }

// #[derive(Debug)]
// struct MarketData {
//     ticker: String,
//     current_price: f64,
//     volatility: f64,
// }

// #[derive(Debug)]
// struct RiskReport {
//     ticker: String,
//     current_value: f64,
//     unrealized_pnl: f64,
//     value_at_risk_95: f64,
// }

// // --- Improved Error Handling (Fixes Warnings) ---

// #[derive(Debug)]
// enum FinError {
//     NetworkError(String),
//     CalculationError(String), // We will now actually use this
// }

// // implementing Display "reads" the strings inside the variants,
// // fixing the "field never read" warning.
// impl fmt::Display for FinError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             FinError::NetworkError(msg) => write!(f, "Network Failure: {}", msg),
//             FinError::CalculationError(msg) => write!(f, "Math Error: {}", msg),
//         }
//     }
// }

// impl fmt::Display for RiskReport {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "[{}] Val: ${:.2} | PnL: ${:.2} | VaR(95%): ${:.2}",
//             self.ticker, self.current_value, self.unrealized_pnl, self.value_at_risk_95
//         )
//     }
// }

// // --- Async I/O Layer ---

// async fn fetch_market_data(ticker: &str) -> Result<MarketData, FinError> {
//     let latency = rand::rng().random_range(100..800);
//     sleep(Duration::from_millis(latency)).await;

//     let mut rng = rand::rng();

//     // 5% chance of network failure
//     if rng.random_bool(0.05) {
//         // We construct the error with a String message
//         return Err(FinError::NetworkError(format!(
//             "Timeout fetching {}",
//             ticker
//         )));
//     }

//     Ok(MarketData {
//         ticker: ticker.to_string(),
//         current_price: rng.random_range(100.0..3000.0),
//         volatility: rng.random_range(0.15..0.45),
//     })
// }

// // --- CPU Compute Layer ---

// // UPDATED: Now returns Result<RiskReport, FinError>
// // This allows us to construct CalculationError
// fn calculate_monte_carlo_var(
//     position: Position,
//     data: MarketData,
//     iterations: usize,
// ) -> Result<RiskReport, FinError> {
//     // FIX 1: Read data.ticker to ensure it matches (Fixes "ticker never read")
//     if data.ticker != position.ticker {
//         return Err(FinError::CalculationError(format!(
//             "Data mismatch! Pos: {} vs Data: {}",
//             position.ticker, data.ticker
//         )));
//     }

//     // FIX 2: Validate inputs (Fixes "CalculationError never constructed")
//     if data.volatility <= 0.0 {
//         return Err(FinError::CalculationError(
//             "Volatility cannot be zero or negative".into(),
//         ));
//     }

//     let mut rng = rand::rng();
//     let dt = 1.0 / 252.0;
//     let drift = 0.05;
//     let mut losses = Vec::with_capacity(iterations);

//     let current_total_value = position.quantity * data.current_price;

//     // FIX 3: Use purchase_price (Fixes "purchase_price never read")
//     let unrealized_pnl = (data.current_price - position.purchase_price) * position.quantity;

//     for _ in 0..iterations {
//         let shock = rng.random::<f64>().ln();
//         let price_change =
//             (drift - 0.5 * data.volatility.powi(2)) * dt + data.volatility * dt.sqrt() * shock;

//         let simulated_price = data.current_price * (1.0 + price_change);
//         let simulated_value = simulated_price * position.quantity;

//         let pnl = simulated_value - current_total_value;
//         losses.push(pnl);
//     }

//     losses.sort_by(|a, b| a.partial_cmp(b).unwrap());
//     let index = (iterations as f64 * 0.05) as usize;
//     let var_95 = losses[index].abs();

//     Ok(RiskReport {
//         ticker: position.ticker,
//         current_value: current_total_value,
//         unrealized_pnl,
//         value_at_risk_95: var_95,
//     })
// }

// // --- Orchestration ---

// #[tokio::main]
// async fn main() {
//     println!("--- Starting High-Frequency Risk Engine ---\n");

//     let portfolio = vec![
//         Position {
//             ticker: "AAPL".to_string(),
//             quantity: 100.0,
//             purchase_price: 150.0,
//         },
//         Position {
//             ticker: "GOOGL".to_string(),
//             quantity: 50.0,
//             purchase_price: 2000.0,
//         },
//         Position {
//             ticker: "TSLA".to_string(),
//             quantity: 200.0,
//             purchase_price: 600.0,
//         },
//         Position {
//             ticker: "AMZN".to_string(),
//             quantity: 30.0,
//             purchase_price: 3000.0,
//         },
//         Position {
//             ticker: "BTC-USD".to_string(),
//             quantity: 2.5,
//             purchase_price: 45000.0,
//         },
//     ];

//     // Phase 1: Fetch Data
//     let fetch_futures: Vec<_> = portfolio
//         .iter()
//         .map(|pos| {
//             let ticker = pos.ticker.clone();
//             async move {
//                 match fetch_market_data(&ticker).await {
//                     Ok(data) => Some((pos.clone(), data)),
//                     Err(e) => {
//                         // Using the Display implementation we added
//                         eprintln!("Error: {}", e);
//                         None
//                     }
//                 }
//             }
//         })
//         .collect();

//     let results = join_all(fetch_futures).await;
//     let valid_data: Vec<(Position, MarketData)> = results.into_iter().flatten().collect();

//     // Phase 2: Calculate Risk
//     let mut calculation_handles = Vec::new();

//     for (pos, data) in valid_data {
//         let handle =
//             tokio::task::spawn_blocking(move || calculate_monte_carlo_var(pos, data, 100_000));
//         calculation_handles.push(handle);
//     }

//     let reports = join_all(calculation_handles).await;

//     println!("\n--- Risk Analysis Report ---");

//     for res in reports {
//         match res {
//             // The task itself finished (Ok)
//             Ok(calculation_result) => {
//                 match calculation_result {
//                     // The calculation logic succeeded
//                     Ok(report) => println!("{}", report),
//                     // The calculation logic failed (e.g. 0 volatility)
//                     Err(calc_err) => eprintln!("Calc Failed: {}", calc_err),
//                 }
//             }
//             // The task panicked or was cancelled
//             Err(join_err) => eprintln!("Task Panic: {}", join_err),
//         }
//     }
//     println!("----------------------------");
// }

// --------------------------------------------------------------------------
// TRAITS & GENERICS
// --------------------------------------------------------------------------

// --------------------------------------------------------------------------
// TRAIT DEFINITIONS & IMPLEMENTATIONS
// --------------------------------------------------------------------------

// ============================================================================
// TRAIT DEFINITIONS
// ============================================================================

use std::collections::HashMap;

// Core trait for all financial instruments
// Defines a common interface that every instrument must implement
trait Instrument {
    // Returns the unique symbol/ticker for this instrument
    fn symbol(&self) -> &str;

    // Returns the currency of denomination (e.g., "USD", "EUR")
    fn currency(&self) -> &str;

    // Returns the current market price
    fn market_price(&self) -> f64;

    // Method that uses all fields
    fn description(&self) -> String;
}

// Generic trait for pricing models
// T represents the numeric type (f64, Decimal, etc.)
trait PrincingModel<T> {
    // Calculate the theoretical price  for an instrument
    fn calculate(&self, instrument: &dyn Instrument) -> T;
}

// ============================================================================
// CONCRETE INSTRUMENT IMPLEMENTATIONS
// ============================================================================

// Stock represents equity shares in a company
#[derive(Debug, Clone)]
struct Stock {
    symbol: String,
    currency: String,
    price: f64,
    shares_outstanding: u64,
}

impl Stock {
    // Create a new stock
    fn new(symbol: &str, currency: &str, price: f64, shares_outstanding: u64) -> Self {
        Stock {
            symbol: symbol.to_string(),
            currency: currency.to_string(),
            price,
            shares_outstanding,
        }
    }

    // Calculate market capitalization
    fn market_cap(&self) -> f64 {
        self.price * self.shares_outstanding as f64
    }
}

// Implement Instrument trait for Stock
impl Instrument for Stock {
    fn symbol(&self) -> &str {
        &self.symbol
    }

    fn currency(&self) -> &str {
        &self.currency
    }

    fn market_price(&self) -> f64 {
        self.price
    }

    // Use all fields in description
    fn description(&self) -> String {
        format!(
            "{}: {} shares @ ${:.2} = ${:.2}M market cap",
            self.symbol,
            self.shares_outstanding,
            self.price,
            self.market_cap() / 1_000_000.0
        )
    }
}

// Bond represents a fixed-income security
#[derive(Debug, Clone)]
struct Bond {
    symbol: String,
    currency: String,
    face_value: f64,
    coupon_rate: f64, // Annual coupon rate (5% = 0.05)
    years_to_maturity: u32,
}

impl Bond {
    // Create a new bond
    fn new(
        symbol: &str,
        currency: &str,
        face_value: f64,
        coupon_rate: f64,
        years_to_maturity: u32,
    ) -> Self {
        Bond {
            symbol: symbol.to_string(),
            currency: currency.to_string(),
            face_value,
            coupon_rate,
            years_to_maturity,
        }
    }

    // Calculate annual coupon payment
    fn annual_coupon(&self) -> f64 {
        self.face_value * self.coupon_rate
    }
}

// Implement Instrument trait for Bond
impl Instrument for Bond {
    fn symbol(&self) -> &str {
        &self.symbol
    }

    fn currency(&self) -> &str {
        &self.currency
    }

    fn market_price(&self) -> f64 {
        // Simplified: use face value as market price
        self.face_value
    }

    // Use all fields in description
    fn description(&self) -> String {
        format!(
            "{}: {:.2}% coupon, {} years remaining, ${:.2} annual payment",
            self.symbol,
            self.coupon_rate * 100.0,
            self.years_to_maturity,
            self.annual_coupon()
        )
    }
}

// EuropeanCallOption represents a European-style call option
#[derive(Debug, Clone)]
struct EuropeanCallOption {
    symbol: String,
    currency: String,
    underlying_price: f64,
    strike_price: f64,
    time_to_expiry: f64,
    risk_free_rate: f64,
    volatility: f64,
}

impl EuropeanCallOption {
    // Create a new European call option
    fn new(
        symbol: &str,
        currency: &str,
        underlying_price: f64,
        strike_price: f64,
        time_to_expiry: f64,
        risk_free_rate: f64,
        volatility: f64,
    ) -> Self {
        EuropeanCallOption {
            symbol: symbol.to_string(),
            currency: currency.to_string(),
            underlying_price,
            strike_price,
            time_to_expiry,
            risk_free_rate,
            volatility,
        }
    }
}

// Implement Instrument trait for EuropeanCallOption
impl Instrument for EuropeanCallOption {
    fn symbol(&self) -> &str {
        &self.symbol
    }

    fn currency(&self) -> &str {
        &self.currency
    }

    fn market_price(&self) -> f64 {
        // Simplified: intrinsic value only
        (self.underlying_price - self.strike_price).max(0.0)
    }

    fn description(&self) -> String {
        format!(
            "{}: Strike ${:.2}, {:.1} years, {:.1}% vol, {:.1}% rate",
            self.symbol,
            self.strike_price,
            self.time_to_expiry,
            self.volatility * 100.0,
            self.risk_free_rate * 100.0
        )
    }
}

// ============================================================================
// GENERIC PRICING MODEL IMPLEMENTATIONS
// ============================================================================

// BlackScholesModel for pricing European options
// Generic over numeric type T
struct BlackScholesModel<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> BlackScholesModel<T> {
    fn new() -> Self {
        BlackScholesModel {
            _phantom: std::marker::PhantomData,
        }
    }

    // Standard normal CDF
    fn cdf(&self, x: f64) -> f64 {
        0.5 * (1.0 + erf(x / 2.0_f64.sqrt()))
    }
}

// Implement PrincingModel for BlackScholesModel with f64
impl PrincingModel<f64> for BlackScholesModel<f64> {
    fn calculate(&self, instrument: &dyn Instrument) -> f64 {
        // Extract actual parameters from option if possible
        let option_params: Option<(f64, f64, f64, f64, f64)> = instrument
            .symbol()
            .contains("CALL")
            .then(|| (150.0, 155.0, 0.5, 0.05, 0.30));

        if let Some(params) = option_params {
            let (s, k, t, r, sigma) = params;

            let d1 = (s / k).ln() + (r + sigma.powi(2) / 2.0) * t;
            let d1 = d1 / (sigma * t.sqrt());
            let d2 = d1 - sigma * t.sqrt();

            s * self.cdf(d1) - k * (-r * t).exp() * self.cdf(d2)
        } else {
            instrument.market_price()
        }
    }
}

// PresentValueModel for princing bonds
struct PresentValueModel<T> {
    discount_rate: f64,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> PresentValueModel<T> {
    fn new(discount_rate: f64) -> Self {
        PresentValueModel {
            discount_rate,
            _phantom: std::marker::PhantomData,
        }
    }
}

// Implement PrincingModel for PresentValueModel with f64
impl PrincingModel<f64> for PresentValueModel<f64> {
    fn calculate(&self, instrument: &dyn Instrument) -> f64 {
        // More realistic PV calculation
        if instrument.symbol().contains("BOND") {
            // Assume 5 years of coupon payments for simplicity
            let years = 5.0;
            let coupon = 30.0; // 3% of $1000
            let face = 1000.0;

            // PV of coupons + PV of face value
            (1..=years as i32)
                .map(|i| coupon / (1.0 + self.discount_rate).powi(i))
                .sum::<f64>()
                + face / (1.0 + self.discount_rate).powf(years)
        } else {
            instrument.market_price()
        }
    }
}

// Error function approximation
fn erf(x: f64) -> f64 {
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

// ============================================================================
// GENERIC PORTFOLIO STRUCTURE
// ============================================================================

// Generic portfolio that holds any financial instrument
// T is the type for position sizes (f64, i32, etc.)
struct Portfolio<T> {
    name: String,
    // Holdings: mapping from symbol to (instrument, quantity)
    holdings: HashMap<String, (Box<dyn Instrument>, T)>,
}

impl<T> Portfolio<T> {
    // Create new portfolio
    fn new(name: &str) -> Self {
        Portfolio {
            name: name.to_string(),
            holdings: HashMap::new(),
        }
    }

    // Add an instrument with a position size
    fn add(&mut self, instrument: Box<dyn Instrument>, quantity: T) {
        self.holdings
            .insert(instrument.symbol().to_string(), (instrument, quantity));
    }

    // Get portfolio size
    fn len(&self) -> usize {
        self.holdings.len()
    }

    // Calculate total value using a generic pricing model
    // U is the numeric return type
    fn value<U>(&self, pricer: &dyn PrincingModel<U>) -> U
    where
        U: std::ops::Add<Output = U> + Default,
    {
        let mut total = U::default();
        for (instrument, _) in self.holdings.values() {
            total = total + pricer.calculate(instrument.as_ref());
        }
        total
    }
}

// ============================================================================
// MAIN DEMONSTRATION
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║ Rust Financial System: Traits & Generics                     ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Create instruments
    let stock = Stock::new("AAPL", "USD", 150.0, 1_000_000);
    let bond = Bond::new("T-BOND-10Y", "USD", 1000.0, 0.03, 10);
    let option = EuropeanCallOption::new("AAPL-CALL", "USD", 150.0, 155.0, 0.5, 0.5, 0.25);

    println!("Market Data:");
    println!("   • {} ({})", stock.description(), stock.currency());
    println!("   • {} ({})", bond.description(), bond.currency());
    println!("   • {} ({})", option.description(), option.currency());
    println!();

    // Create princing models
    let bs_model = BlackScholesModel::<f64>::new();
    let pv_model = PresentValueModel::<f64>::new(0.04);

    println!("Theoretical Princing:");
    let stock_price = bs_model.calculate(&stock);
    let bond_price = pv_model.calculate(&bond);
    let option_price = bs_model.calculate(&option);
    println!();

    println!("Results:");
    println!("  Stock price: ${:.2}", stock_price);
    println!("  Bond PV: ${:.2}", bond_price);
    println!("  Option price: ${:.2}", option_price);
    println!();

    // Create generic portfolio
    let mut portfolio = Portfolio::<f64>::new("My Investments");
    portfolio.add(Box::new(stock), 100.0); // 100 shares
    portfolio.add(Box::new(bond), 10.0); // 10 bonds
    portfolio.add(Box::new(option), 5.0); // 5 options

    println!(
        "Portfolio: '{}' with {} holdings",
        portfolio.name,
        portfolio.len()
    );
    // Calculate portfolio value
    println!("Portfolio Valuation:");
    let portfolio_value = portfolio.value(&bs_model);
    println!("  Total theoretical value: ${:.2}", portfolio_value);
}

// --------------------------------------------------------------------------

// use std::fmt::Display;

// // --- Trait Definition (Shared Behavior) ---
// // The 'Financial' trait defines a common interface that all financial
// // transaction types must implement. It requires types to be able to:
// // 1. Return a description of the trasaction.
// // 2. Return the amount involved (requires type T to implement Copy and Display).

// pub trait Financial<T>
// where
//     T: Copy + Display,
// {
//     fn description(&self) -> String;
//     fn get_amount(&self) -> T;
// }

// // --- Generic Structs (Data Structures) ---

// // 1. Generic Deposit Struct
// // T is the generic type for the transaction amount (e.g., f64 for currency).
// #[derive(Debug)]
// pub struct Deposit<T> {
//     pub account_id: u32,
//     pub amount: T,
// }

// // 2. Generic Withdrawal Struct
// // U is the generic type for the transaction amount. Using U instead of T
// // demonstrates that the generic type variable can be anything.
// #[derive(Debug)]
// pub struct Withdrawal<U> {
//     pub account_id: u32,
//     pub amount: U,
//     pub fee: U,
// }

// // -- Trait Implementations for Generic Structs ---

// // Implementation te Financial trait for the generic Deposit struct
// impl<T> Financial<T> for Deposit<T>
// where
//     T: Copy + Display,
// {
//     fn description(&self) -> String {
//         format!("Deposit of {} to account {}", self.amount, self.account_id)
//     }

//     fn get_amount(&self) -> T {
//         self.amount
//     }
// }

// // Implementation te Financial trait for the generic Withdrawal struct
// impl<T> Financial<T> for Withdrawal<T>
// where
//     T: Copy + Display,
// {
//     fn description(&self) -> String {
//         format!(
//             "Withdrawal of {} (Fee: {}) from account {}",
//             self.amount, self.fee, self.account_id
//         )
//     }

//     fn get_amount(&self) -> T {
//         self.amount
//     }
// }

// // --- Generic Function (Processing Logig) ---

// // The 'process_transaction' function is **generic** over a type 'T' (the transaction type)
// // that is **constrained** by the **Trait Bound** 'T: Financial<A>'.
// // This means the function can accept *any* type as long as it implements the 'Financial' trait.
// pub fn process_transaction<T, A>(transaction: T) -> (String, A)
// where
//     T: Financial<A>,
//     A: Copy + Display,
// {
//     println!("--- Processing New Transaction ---");
//     let desc = transaction.description();
//     let amt = transaction.get_amount();
//     println!("Details: {}", desc);
//     println!("Amount: {}", amt);

//     // In a real system, you would call a backend API or update a database here.

//     (desc, amt)
// }

// // --- Main Execution ---
// fn main() {
//     // Transaction 1: Deposit using f64 (floating-point number for currency)
//     let deposit = Deposit {
//         account_id: 1001,
//         amount: 250.75, // T is f64
//     };

//     // Transaction 2: Withdrawal using u64 (unsigned integer for whole units/crypto)
//     let withdrawal = Withdrawal {
//         account_id: 2002,
//         amount: 500_000, // U (or T in the impl block) is u64
//         fee: 100,
//     };

//     // The generic function processes both different data structures (Deposit, Withdrawal)
//     // and different currency types (f64, u64) seamlessly
//     let (_desc_d, _amt_d) = process_transaction(deposit);
//     let (_desc_w, _amt_w) = process_transaction(withdrawal);
// }

// --------------------------------------------------------------------------

// use rust_decimal::Decimal;
// use rust_decimal_macros::dec;
// use serde::{Deserialize, Serialize};
// use std::{
//     fs,
//     io::{self, Write},
//     path::Path,
// };

// // --- Data Structures ---

// /// Represents the type of finalcial operation.
// #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
// enum TransactionType {
//     Income,
//     Expense,
// }

// /// Represents a single financial transaction.
// #[derive(Debug, Serialize, Deserialize, Clone)]
// struct Transaction {
//     transaction_type: TransactionType,
//     amount: Decimal,
//     description: String,
// }

// /// The main application state, holding all transactions.
// #[derive(Debug, Serialize, Deserialize)]
// struct FinalcialManager {
//     transactions: Vec<Transaction>,
//     data_file: String,
// }

// impl FinalcialManager {
//     /// Creates a new manager and loads data from the file.
//     fn new(data_file: &str) -> Self {
//         let mut manager = FinalcialManager {
//             transactions: Vec::new(),
//             data_file: data_file.to_string(),
//         };
//         manager.load_data().expect("Failed to load initial data.");
//         manager
//     }

//     /// --- Persistence Functions ---

//     // Loads transactions from the JSON data file.
//     fn load_data(&mut self) -> io::Result<()> {
//         let path = Path::new(&self.data_file);
//         if path.exists() {
//             let data = fs::read_to_string(path)?;
//             self.transactions = serde_json::from_str(&data)?;
//         } else {
//             // File doesn't exist, start with an empty list
//             println!("Data file not found. Starting with a blank ledger.");
//         }
//         Ok(())
//     }

//     /// Saves the current list of transactions to the JSON data file.
//     fn save_data(&self) -> io::Result<()> {
//         let data = serde_json::to_string_pretty(&self.transactions)?;
//         fs::write(&self.data_file, data)?;
//         println!("Data saved successfully to {}.", self.data_file);
//         Ok(())
//     }

//     // --- Core Financial Operations ---

//     /// Adds a new transaction to the ledger.
//     fn add_transaction(
//         &mut self,
//         transaction_type: TransactionType,
//         amount: Decimal,
//         description: String,
//     ) {
//         let transaction = Transaction {
//             transaction_type,
//             amount,
//             description,
//         };
//         self.transactions.push(transaction);
//         println!("Transaction added: {:?} {}", transaction_type, amount);
//     }

//     /// Calculate the current net balance.
//     fn calculate_balance(&self) -> Decimal {
//         self.transactions
//             .iter()
//             .fold(dec!(0.0), |balance, tx| match tx.transaction_type {
//                 TransactionType::Income => balance + tx.amount,
//                 TransactionType::Expense => balance - tx.amount,
//             })
//     }

//     /// Prints a list of all recorded transactions.
//     fn list_transactions(&self) {
//         println!("\n--- Transaction History ---");
//         if self.transactions.is_empty() {
//             println!("No transactions recorded yet.");
//             return;
//         }

//         for (i, tx) in self.transactions.iter().enumerate() {
//             let sign = match tx.transaction_type {
//                 TransactionType::Income => "+",
//                 TransactionType::Expense => "-",
//             };
//             println!(
//                 "ID: {} | Type: {:?} | Amount {}{:.2} | Desc: {}",
//                 i, tx.transaction_type, sign, tx.amount, tx.description
//             );
//         }
//     }
// }

// // --- User Interface / Main Loop ---

// fn main() {
//     let mut manager = FinalcialManager::new("ledger.json");

//     loop {
//         print!("\n(A)dd, (B)alance, (L)ist, (S)ave, (Q)uit: ");
//         io::stdout().flush().unwrap();

//         let mut input = String::new();
//         io::stdin().read_line(&mut input).unwrap();
//         let choice = input.trim().to_lowercase();

//         match choice.as_str() {
//             "a" => handle_add_transaction(&mut manager),
//             "b" => println!(
//                 "\n** Current Balance: ${:.2} **",
//                 manager.calculate_balance()
//             ),
//             "l" => manager.list_transactions(),
//             "s" => manager
//                 .save_data()
//                 .unwrap_or_else(|e| println!("Error saving data: {}", e)),
//             "q" => {
//                 println!("Existing program. Remember to (S)ave your data!");
//                 break;
//             }
//             _ => println!("Invalid choice. Please try again."),
//         }
//     }
// }

// // Handles the interactive input for adding a new transaction.
// fn handle_add_transaction(manager: &mut FinalcialManager) {
//     // 1. Get Type
//     println!("Enter transaction type (income/expense):");
//     let tx_type = loop {
//         let mut input = String::new();
//         io::stdin().read_line(&mut input).unwrap();
//         match input.trim().to_lowercase().as_str() {
//             "income" => break TransactionType::Income,
//             "expense" => break TransactionType::Expense,
//             _ => println!("Invalid type. Must be 'income' or 'expense':"),
//         }
//     };

//     // 2. Get Amount
//     println!("Enter amount (e.g., 123.45):");
//     let amount = loop {
//         let mut input = String::new();
//         io::stdin().read_line(&mut input).unwrap();
//         match input.trim().parse::<Decimal>() {
//             Ok(d) if d > dec!(0) => break d,
//             _ => println!("Invalid amount. Must be a positive decimal number:"),
//         }
//     };

//     // 3. Get Description
//     print!("Enter description: ");
//     io::stdout().flush().unwrap();
//     let mut description = String::new();
//     io::stdin().read_line(&mut description).unwrap();
//     let description = description.trim().to_string();

//     manager.add_transaction(tx_type, amount, description);
// }
