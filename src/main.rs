#![allow(non_snake_case)]
use bdk::bitcoincore_rpc::{Auth as rpc_auth, Client, RpcApi};
use std::io::{self, Write};

fn main() {
    let rpc_auth = rpc_auth::CookieFile("/home/orangepi/.bitcoin/.cookie".into());

    let mut input = String::new();
    print!("Please enter wallet label: ");
    io::stdout().flush().unwrap(); // Make sure the prompt is immediately displayed
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();
    let url = format!("http://127.0.0.1:18443/wallet/{}", input);

    let core_rpc = Client::new(&url, rpc_auth).unwrap();

    core_rpc.create_wallet(&input, None, None, None, None).unwrap();

    println!("Successfully created wallet: {}", input);
}
