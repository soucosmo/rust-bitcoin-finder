use std::collections::BTreeMap;
use files::{
    read_last_key,
    write_to_file,
    write_last_key,
};
mod wallets;
mod wallet;
mod config;
mod files;
mod utils;
use num_traits::{FromPrimitive, ToPrimitive};
use num_bigint::BigUint;
use secp256k1::Secp256k1;
use utils::{
    get_target_hash_from_address,
    verify_address_with_murmur,
    generate_wif,
};

fn main() {
    // Carrega as carteiras
    let wallets_map: BTreeMap<u8, wallet::Wallet> = wallets::wallets();
    let min = wallets_map.keys().min().unwrap();
    let max = wallets_map.keys().max().unwrap();

    // Solicita ao usuário que selecione uma wallet
    let mut user_choice = String::new();
    loop {
        println!("Selecione o número da wallet para realizar a comparação: ({min} - {max})");

        user_choice.clear();

        std::io::stdin().read_line(&mut user_choice).expect("Falha ao ler a linha");

        let user_choice: u8 = match user_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, insira um número válido.");
                continue;
            }
        };

        if wallets_map.contains_key(&user_choice) {
            break;
        } else {
            println!("Wallet não encontrada. Escolha um número válido.");
        }
    }

    let user_choice = user_choice.trim().parse::<u8>().unwrap();

    let selected_wallet = wallets_map.get(&user_choice).unwrap();

    // Inicializa a chave privada a partir do arquivo ou com um valor pequeno para demonstração
    let min = read_last_key(user_choice).unwrap_or_else(|| selected_wallet.min.clone());

    // Temporizador para calcular chaves por segundo
    let start_time = std::time::Instant::now();

    // Contador de chaves a partir do primitivo i32 com o valor 0
    let mut key_count = BigUint::from_i32(0).unwrap();

    // Temporizador para salvar o último hexadecimal a cada 5 minutos
    let mut last_save_time = std::time::Instant::now();

    // necessário converter o endereço bitcoin para a target hash
    // assim a comparação irá reduzir algumas etapas.
    let address_target_hash = get_target_hash_from_address(&selected_wallet.address);

    let secp = Secp256k1::new();

    for priv_key in num_iter::range_inclusive(min, selected_wallet.max.clone()) {
        //println!("Contador de chaves: {}", count);

        key_count += 1u8;

        if verify_address_with_murmur(&priv_key, address_target_hash, &secp) {
            eprintln!("Correspondência encontrada! Endereço: {}", selected_wallet.address);

            // Gera WIF a partir da chave privada atual
            let wif = generate_wif(&priv_key);

            let data_hora = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let content = format!(
                "Chave privada: {:064x}\nWIF: {}\nEndereço Bitcoin: {}\nData e Hora: {}\n",
                priv_key, wif, selected_wallet.address, data_hora
            );

            let file_name = format!("{user_choice}_found.txt");

            write_to_file(
                &file_name,
                &content
            );

            // Salva o último hexadecimal ao finalizar
            write_last_key(&priv_key, user_choice);

            break;
        }

        // Verifica se 5 minutos se passaram para salvar o último hexadecimal
        if last_save_time.elapsed() >= std::time::Duration::from_secs(300) {
            write_last_key(&priv_key, user_choice);
            last_save_time = std::time::Instant::now();

            // Calcula chaves por segundo
            let elapsed_time = start_time.elapsed().as_secs_f64();

            // provável overflow aqui no futuro para números maiores que (2^64) - 1
            let keys_per_second = key_count.to_f64().unwrap() / elapsed_time;

            println!("Chaves processadas: {}", key_count);
            println!("Chaves por segundo: {:.2}", keys_per_second);
        }
    }

    // Calcula chaves por segundo
    let elapsed_time = start_time.elapsed().as_secs_f64();

    // provável overflow aqui no futuro para números maiores que (2^64) - 1
    let keys_per_second = key_count.to_f64().unwrap() / elapsed_time;

    println!("Chaves processadas: {}", key_count);
    println!("Tempo total: {:.2} segundos", elapsed_time);
    println!("Chaves por segundo: {:.2}", keys_per_second);

    println!("Processo concluído.");
}
