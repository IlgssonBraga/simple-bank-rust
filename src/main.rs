mod client;

fn main() {

    let mut c1 = client::Client::create_client(String::from("Ilgsson"), String::from("07811768402"));

    c1.set_name(String::from("Ilgner"));
    c1.set_cpf(String::from("12345678910"));

    println!("{}", c1.get_name());
    println!("{}", c1.get_cpf());
}
