
pub fn primitive_data_type(){
    
    println!("\n");
    let token_supply: u128 = 1_000_000_000_000;
    let block_number: i64 = -123456789;

    println!("Token Supply (u128): {}", token_supply);
    println!("Block Number (i64): {}", block_number);


    let token_price: f32 = 3.14;
    let transaction_fee: f64 = 0.000001;

    println!("Token Price (f32): {}", token_price);
    println!("Transaction Fee (f64): {}", transaction_fee);


    let is_transaction_valid: bool = true;
    println!("Is the transaction valid? {}", is_transaction_valid);

    let token_symbol: char = 'T';
    println!("Token Symbol: {}", token_symbol);


    let wallet_address: &str = "0x123444849e9ADEFGRT";

    let contract_name: String = String::from("Decentralized Exchange");


    println!("Wallet Address: {}", wallet_address);
    println!("Contract Name: {}", contract_name);


}


pub fn arithmetic_operations(){
    let account_balance: i32 = 1000;
    let transaction_amount: i32 = 250;


    // operations
    println!(
        "Account Balance: {}, Transaction Amount: {}",
        account_balance,transaction_amount
             
    );

    println!(
        "New Account Balance after transaction: {}",
        account_balance - transaction_amount
    ); // Subtraction transaction

    println!(
        "Doubling transaction amount (for staking): {}",
        transaction_amount * 2
    ); // multiplication for staking rewards

    println!(
        "Division for share distribution: 1000/4 = {}",
        account_balance / 4
    ); //  Example of distributing funds between 4 participants

    println!(
        "Remainder when dividing transaction fee: 1000 % 3 = {}",
        account_balance % 3
    );

    let gas_price: f64 = 0.0000000012;
    let gas_used: f64 = 21000.0;
    println!("Gas Price = {}, Gas Used = {}", gas_price, gas_used);
    println!("Total Gas fee: {:.8}", gas_price * gas_used);





}



pub fn logical_operations(){
    let is_staking: bool = true;
    let has_sufficient_balance: bool = false;

    println!(
        "Is Staking = {}, Has Sufficient Balance = {}",
        is_staking, has_sufficient_balance
    );

    println!(
        "Can Perform Staking: {}",
        is_staking && has_sufficient_balance
    );

    println!(
        "Can Either Perform Staking or withdrawal: {}",
        is_staking || has_sufficient_balance
    );

    println!("Negating Staking Status: !is_staking = {}", !is_staking);

}


pub fn  variable_shadowing_and_conversion(){
    let account_balance: i32 = 500;
    println!("Initial Account Balance: {}", account_balance);

    let account_balance = account_balance + 100;
    println!("Updated Account Balance: {}", account_balance);

    let gas_fee: f64 = 0.0025;
    let gas_fee_int: i32 = gas_fee as i32;

    print!("Gas Fee (f64): {}, Converted to lanports: {}",
            gas_fee, gas_fee_int
    );

    let block_height: i32 = 123456;
    let block_heigh_str: String = block_height.to_string(); 
    println!(
        "Block Height: {}, Converted to string: {}",
        block_height, block_heigh_str
    );


}

pub fn mutability_example(){
    
    let token_supply: i32 = 1_000_000;


    let mut user_balance: i32 = 500;
    println!("Before transaction: User Balance = {}", user_balance);

    user_balance -= 50;
    println!("After transaction: User Balance = {}", user_balance);

}


pub fn tuple_destructuring_example(){
    let transaction_info: (&str, i32, f64) = ("Transfer", 200, 0.002);  
    let (tx_type, tx_amount, tx_fee) = transaction_info;

    // destructuring method for accessing tuple data
    println!(
        "Transaction Type: {}, Amount: {}, Fee: {}",
        tx_type, tx_amount, tx_fee
    );
    // or numbering method
    println!(
        "Transaction Type: {}, Amount: {}, Fee: {}",
        transaction_info.0, transaction_info.1, transaction_info.2
    );

}


    pub fn demo(){
    println!("\n");
    primitive_data_type();

    println!("\n");
    arithmetic_operations();

    println!("\n");
    logical_operations();

    print!("\n");
    variable_shadowing_and_conversion();

    print!("\n");
    mutability_example();

    print!("\n");
    tuple_destructuring_example();
}