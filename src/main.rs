mod client;
mod account;

fn main() {

    let c1 = client::Client::create_client(String::from("Ilgsson"), String::from("07811768402"));

    let account1 = account::Account::create_account(1234, 223531, 0);

    println!("{}", c1.get_name());
    println!("{}", account1.get_number());
    println!("{}", c1.get_cpf());
}
