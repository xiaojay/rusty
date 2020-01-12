use ring::aead;
use base64;
use rand::{thread_rng, Rng};

fn main() {
    let text = "hello world";
    let mut to_encrypt = String::from(text).as_bytes().to_vec();
    let key = [0; 32];
    let sealing_key = aead::SealingKey::new(&aead::AES_256_GCM, &key).expect("Failed get key");
    let nonce: [u8; 12] = thread_rng().gen();
    let suffix_len = aead::AES_256_GCM.tag_len();
    for _ in 0..suffix_len {
        to_encrypt.push(0);
    };
    aead::seal_in_place(&sealing_key, &nonce, &[], &mut to_encrypt, suffix_len);
    println!("text: {:?}", text);
    println!("key: {:?}", key);
    println!("nonce: {:?}", nonce);
    println!("encrypted: {}", base64::encode(&to_encrypt))
}
