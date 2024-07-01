use k256::{
    ecdsa::SigningKey, 
    elliptic_curve::sec1::ToEncodedPoint
};
use num_bigint::BigInt;


// Função para converter chave privada BigInt para chave pública
pub fn private_key_to_public_key(private_key: &BigInt) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Converta BigInt para array de bytes (ignorando o sinal)
    let (_sign, priv_key_bytes) = private_key.to_bytes_be();
    
    // Chave privada deve ter exatamente 32 bytes para ser válida
    let priv_key_bytes_padded = if priv_key_bytes.len() < 32 {
        let mut padded = vec![0u8; 32 - priv_key_bytes.len()];
        padded.extend_from_slice(&priv_key_bytes);
        padded
    } else {
        priv_key_bytes
    };

    // Crie a chave de assinatura a partir da chave privada
    let signing_key = SigningKey::from_bytes(&priv_key_bytes_padded)?;

    // Derive a chave pública
    let verifying_key = signing_key.verifying_key();

    // Converta a chave pública para bytes (formato comprimido)
    let public_key = verifying_key.to_encoded_point(true);

    // Retorne a chave pública como vetor de bytes
    Ok(public_key.as_bytes().to_vec())
}