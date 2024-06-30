mod btc_utils;
mod base58;
mod wallet;
mod wallets;
use crate::base58::encode;
use btc_utils::{generate_wif, create_public_hash160};


fn main() {
    // resultado esperado para a carteira 2 do puzzle
    // KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU74sHUHy8S
    let priv_key_int = "0000000000000000000000000000000000000000000000000000000000000003";

    match generate_wif(priv_key_int) {
        Ok(wif) => println!("WIF: {}", wif),
        Err(e) => println!("Erro ao gerar WIF: {}", e),
    }

    let addr = "1CUNEBjYrCn2y1SdiUMohaKUi4wpP326Lb";

    let pke = [2, 249, 48, 138, 1, 146, 88, 195, 16, 73, 52, 79, 133, 248, 157, 82, 41, 181, 49, 200, 69, 131, 111, 153, 176, 134, 1, 241, 19, 188, 224, 54, 249];

    match create_public_hash160(priv_key_int) {
        Ok(pub_hash) => {
            println!("Public Hash160: {:?}", pub_hash);

            let a = encode(&pub_hash);
            println!("a: {}", a);
            for i in &pke {
                //print!("{:02x}", i);
                match generate_wif(format!("{:02x}", i).as_str()) {
                    Ok(wif) => {
                        //println!(" WIF: {}", &wif);

                        if addr.contains(&wif) {
                            println!("encontrou  a chave");
                        } else {
                            println!("nao encontrou a chave");
                        }
                    },
                    Err(e) => println!("Erro ao gerar WIF: {}", e),
                }
            }
        },
        Err(e) => println!("Erro na criação da public hash160: {}", e),
    }
}