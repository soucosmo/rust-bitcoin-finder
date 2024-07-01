use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::thread;
use std::time::{Instant, Duration};

mod btc_utils;
mod wallets;
mod wallet;

use btc_utils::{
    generate_wif,
    private_key_to_public_key,
    public_key_to_address,
    read_last_key,
    write_to_file,
    write_last_key,
};

use num_bigint::BigInt;

const CORRESPONDING_KEYS_FILE: &str = "corresponding_keys.txt";
const BATCH_SIZE: usize = 5000;

fn main() {
    // Carrega as carteiras
    let wallets_map: BTreeMap<u8, wallet::Wallet> = wallets::wallets();
    let min = *wallets_map.keys().min().unwrap();
    let max = *wallets_map.keys().max().unwrap();

    // Solicita ao usuário que selecione uma wallet
    let mut user_choice = String::new();
    let selected_wallet_address;
    let selected_wallet;

    loop {
        println!("Selecione o número da wallet para realizar a comparação: ({} - {})", min, max);
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
            selected_wallet = wallets_map.get(&user_choice).unwrap().clone();
            selected_wallet_address = selected_wallet.address.clone();
            break;
        } else {
            println!("Wallet não encontrada. Escolha um número válido.");
        }
    }

    // Inicializa a chave privada a partir do arquivo ou com um valor pequeno para demonstração
    let priv_key = read_last_key().unwrap_or_else(|| BigInt::from(1));

    // Temporizador para calcular chaves por segundo
    let start_time = Instant::now();
    
    // Contador atômico para o número total de chaves processadas
    let total_key_count = Arc::new(AtomicUsize::new(0));

    // Número de threads a serem usadas
    let num_threads = num_cpus::get(); // ou defina um número específico
    println!("Número de threads: {}", num_threads);

    // Cria um canal para comunicação entre as threads e o thread principal
    let (tx, rx) = std::sync::mpsc::channel();

    // Compartilha a chave privada entre as threads
    let priv_key = Arc::new(Mutex::new(priv_key.clone()));

    // Flag para sinalizar quando uma correspondência é encontrada
    let found = Arc::new(AtomicBool::new(false));

    // Calcula o intervalo para cada thread
    let keys_per_thread = (1..=BATCH_SIZE).step_by(num_threads);
    let mut threads = vec![];

    // Variável para armazenar o total de chaves processadas por todas as threads
    let total_processed_count = Arc::new(Mutex::new(0));

    for (thread_index, key_start) in keys_per_thread.enumerate() {
        let key_end = key_start + BATCH_SIZE - 1;
        let mut thread_batch = Vec::new();
        
        // Gera as chaves privadas para este lote de threads
        for _ in key_start..=key_end {
            if found.load(Ordering::SeqCst) {
                break;
            }
            let priv_key_clone = priv_key.lock().unwrap().clone();
            thread_batch.push(priv_key_clone);
            *priv_key.lock().unwrap() += BigInt::from(1);
        }
        
        let (tx, selected_wallet_address, total_key_count, found, total_processed_count) = (
            tx.clone(),
            selected_wallet_address.clone(),
            Arc::clone(&total_key_count),
            Arc::clone(&found),
            Arc::clone(&total_processed_count),
        );
        
        let handle = thread::spawn(move || {
            let mut local_key_count = 0;
            for priv_key in &mut thread_batch {
                let priv_key_hex = format!("{:064x}", priv_key);
            
                let wif_key = match generate_wif(&priv_key_hex) {
                    Ok(wif) => wif,
                    Err(e) => {
                        eprintln!("Erro ao gerar WIF: {}", e);
                        continue; // Ou maneje o erro de outra forma
                    }
                };
            
                match private_key_to_public_key(priv_key) {
                    Ok(public_key) => {
                        match public_key_to_address(&public_key) {
                            Ok(address) => {
                                if address == selected_wallet_address {
                                    // Atômico verifica e define a flag found
                                    if !found.swap(true, Ordering::SeqCst) {
                                        println!("Correspondência encontrada! Endereço: {}", address);
                                        let data_hora = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                                        let content = format!(
                                            "Chave privada: {}\nWIF: {}\nEndereço Bitcoin: {}\nData e Hora: {}\n",
                                            priv_key_hex, wif_key, address, data_hora
                                        );
            
                                        // Escreve para o arquivo
                                        write_to_file(CORRESPONDING_KEYS_FILE, &content);
                                        // Envia sinal para o canal indicando a correspondência encontrada
                                        if let Err(e) = tx.send(()) {
                                            eprintln!("Erro ao enviar sinal para o canal: {:?}", e);
                                        } else {
                                            println!("Sinal enviado para o canal.");
                                        }
                                        break; // Sai do loop ao encontrar correspondência
                                    }
                                }
                            }
                            Err(e) => eprintln!("Erro ao converter chave pública para endereço: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Erro ao converter chave privada para chave pública: {}", e),
                }
                local_key_count += 1;
                total_key_count.fetch_add(1, Ordering::SeqCst); // Incrementa o contador total
            }
            
            

            // Atualiza o contador de chaves processadas por todas as threads
            let mut total_processed_count = total_processed_count.lock().unwrap();
            *total_processed_count += local_key_count;
        });
    
        threads.push(handle);
    }

    // Aguarda a primeira correspondência e encerra as threads
    for thread in threads {
        if found.load(Ordering::SeqCst) {
            break;
        }
        if let Err(e) = thread.join() {
            eprintln!("Erro ao aguardar thread: {:?}", e);
        }
    }

    // Calcula chaves por segundo
    let elapsed_time = start_time.elapsed().as_secs_f64();
    let total_processed_count = *total_processed_count.lock().unwrap();
    let keys_per_second = total_processed_count as f64 / elapsed_time;
    
    println!("Total de chaves processadas por todas as threads: {}", total_processed_count);
    println!("Tempo total: {:.2} segundos", elapsed_time);
    println!("Chaves por segundo: {:.2}", keys_per_second);
    
    // Salva a última chave privada processada no arquivo
    write_last_key(&priv_key.lock().unwrap());
    
    println!("Processo concluído.");
}
