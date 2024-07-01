use btc_utils::private_key_to_public_key;
use btc_utils::public_key_to_address;
use btc_utils::generate_wif;
use num_bigint::BigInt;
use num_traits::Num;
mod btc_utils;
mod wallets;
mod base58;
mod wallet;

fn main() {
    // Exemplo de chave privada como hexadecimal
    let priv_key_hex = "0000000000000000000000000000000000000000000000000000000000000003";

    // Wif a partir da chave privada
    match generate_wif(priv_key_hex) {
        Ok(wif) => println!("WIF: {}", wif),
        Err(e) => println!("Erro ao gerar WIF: {}", e),
    }

    // Converta a chave privada para BigInt
    let priv_key = BigInt::from_str_radix(priv_key_hex, 16).unwrap();

    // Converta chave privada para chave pública
    match private_key_to_public_key(&priv_key) {
        Ok(public_key) => {
            // Converta chave pública para endereço Bitcoin
            match public_key_to_address(&public_key) {
                Ok(address) => println!("Endereço Bitcoin: {}", address),
                Err(e) => eprintln!("Erro ao converter chave pública para endereço: {}", e),
            }
        },
        Err(e) => eprintln!("Erro ao converter chave privada para chave pública: {}", e),
    }
}
