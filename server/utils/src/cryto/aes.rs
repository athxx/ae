// use aes::{Aes128, BlockCipher, NewBlockCipher};
// use rand::Rng;

// fn aes_encrypt(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
//     let mut rng = rand::thread_rng();
//     let iv: [u8; 16] = rng.gen();

//     let cipher = Aes128::new(key.into());
//     let mut block = iv;
//     let mut ciphertext = Vec::with_capacity(plaintext.len() + 16);
//     ciphertext.extend_from_slice(&iv);

//     for chunk in plaintext.chunks(16) {
//         block = cipher.encrypt_block(BlockCipher::block_xor(&block, chunk));
//         ciphertext.extend_from_slice(&block);
//     }

//     ciphertext
// }

// fn aes_decrypt(key: &[u8], ciphertext: &[u8]) -> Option<Vec<u8>> {
//     let iv = &ciphertext[..16];
//     let cipher = Aes128::new(key.into());
//     let mut block = iv.clone();
//     let mut plaintext = Vec::with_capacity(ciphertext.len() - 16);

//     for chunk in ciphertext[16..].chunks(16) {
//         if chunk.len() != 16 {
//             return None;
//         }

//         let decrypted_block = BlockCipher::block_xor(&cipher.decrypt_block(chunk), &block);
//         plaintext.extend_from_slice(&decrypted_block);
//         block = chunk.into();
//     }

//     Some(plaintext)
// }
