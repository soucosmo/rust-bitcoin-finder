mod btc_utils;
mod base58;
mod wallet;
mod wallets;

use btc_utils::{generate_wif, create_public_hash160};


fn main() {
    // resultado esperado para a carteira 2 do puzzle
    // KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU74sHUHy8S
    let priv_key_int = "0000000000000000000000000000000000000000000000000000000000000003";

    match generate_wif(priv_key_int) {
        Ok(wif) => println!("WIF: {}", wif),
        Err(e) => println!("Erro ao gerar WIF: {}", e),
    }

    match create_public_hash160(priv_key_int) {
        Ok(pub_hash) => println!("Public Hash160: {:?}", pub_hash),
        Err(e) => println!("Erro na criação da public hash160: {}", e),
    }
}