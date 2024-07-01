use std::fs::create_dir_all;
use std::path::Path;


pub fn make_dir(path: &str) {
    let path = path.as_ref();

    if !Path::exists(path) {
        println!("{} não existe.", path.to_str().unwrap());

        match create_dir_all(path) {
            Ok(_) => println!("Diretório criado com sucesso."),
            Err(e) => panic!("Erro ao criar diretório: {}", e),
        }
    }
}
