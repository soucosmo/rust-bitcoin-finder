use bitcoin::util::key::PublicKey;
use bitcoin::util::address::Address;
use bitcoin::network::constants::Network;


// Função para converter chave pública para endereço Bitcoin
pub fn public_key_to_address(public_key: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    // Crie a chave pública Bitcoin
    let public_key = PublicKey::from_slice(public_key)?;

    // Crie o endereço Bitcoin
    let address = Address::p2pkh(&public_key, Network::Bitcoin);

    // Retorne o endereço Bitcoin como string
    Ok(address.to_string())
}

