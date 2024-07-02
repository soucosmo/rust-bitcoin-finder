use btc_utils::{
    public_key_to_address,
    increment_and_format_key,
    private_key_to_public_key
};
use files::{
    read_last_key,
    write_to_file,
    write_last_key,
};
use std::collections::BTreeMap;
use btc_utils::generate_wif;
use num_bigint::BigInt;
use num_traits::Num;

mod btc_utils;
mod wallets;
mod base58;
mod wallet;
mod config;
mod files;
use crate::base58::decode;

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
    let mut priv_key = read_last_key(user_choice).unwrap_or_else(|| BigInt::from(1));

    // Temporizador para calcular chaves por segundo
    let start_time = std::time::Instant::now();
    let mut key_count = 0;

    // Temporizador para salvar o último hexadecimal a cada 5 minutos
    let mut last_save_time = std::time::Instant::now();
    let mut count = 0;

    // necessário converter o endereço bitcoin para a chave hexadecimal
    // assim a comparação irá reduzir algumas etapas.

    let address_base58 = selected_wallet.address.clone();

    let address_hex = decode(&address_base58.as_str());

    loop {
        // Converte a chave privada atual para hexadecimal formatado
        let priv_key_hex = format!("{:064x}", priv_key);

        //println!("Contador de chaves: {}", count);
        count += 1;
        // Converte chave privada para chave pública
        match private_key_to_public_key(&priv_key) {
            Ok(public_key) => {
                // Converte chave pública para endereço Bitcoin
                match public_key_to_address(&public_key) {
                    // Verifica se o endereço corresponde à wallet selecionada

                    Ok(address) if address == selected_wallet.address => {
                        println!("Public key original: {:?}", public_key);

                        println!("Public key selecionado: {:?}", address_hex);
                        // Gera WIF a partir da chave privada atual
                        let wif_key = match generate_wif(&priv_key_hex) {
                            Ok(wif) => wif,                      // Retorna wif se Ok
                            Err(e) => {
                                eprintln!("Erro ao gerar WIF: {}", e);
                                return;  // Retorna vazio se Err, ou pode tratar de outra maneira
                            },
                        };
                            eprintln!("Correspondência encontrada! Endereço: {}", address);
                            let data_hora = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                            let content = format!(
                                "Chave privada: {}\nWIF: {}\nEndereço Bitcoin: {}\nData e Hora: {}\n",
                                priv_key_hex, wif_key, address, data_hora
                            );

                            let file_name = format!("{user_choice}_found.txt");

                            write_to_file(
                                &file_name,
                                &content
                            );

                            break;
                    },
                    Ok(_address) => {},
                    Err(e) => eprintln!("Erro ao converter chave pública para endereço: {}", e),
                }
            },
            Err(e) => eprintln!("Erro ao converter chave privada para chave pública: {}", e),
        }

        // Incrementa a chave privada para a próxima iteração
        priv_key = BigInt::from_str_radix(&increment_and_format_key(&mut priv_key), 16).unwrap();
        key_count += 1;

        // Verifica se 5 minutos se passaram para salvar o último hexadecimal
        if last_save_time.elapsed() >= std::time::Duration::from_secs(300) {
            write_last_key(&priv_key, user_choice);
            last_save_time = std::time::Instant::now();
        }
    }

    // Calcula chaves por segundo
    let elapsed_time = start_time.elapsed().as_secs_f64();
    let keys_per_second = key_count as f64 / elapsed_time;
    println!("Chaves processadas: {}", key_count);
    println!("Tempo total: {:.2} segundos", elapsed_time);
    println!("Chaves por segundo: {:.2}", keys_per_second);

    // Salva o último hexadecimal ao finalizar
    write_last_key(&priv_key, user_choice);

    println!("Processo concluído.");
}
