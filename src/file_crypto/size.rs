use super::{FileCrypto, DECRYPT_CHUNK_SIZE, ENCRYPT_CHUNK_SIZE, HALF_KEY_LENGTH};

impl FileCrypto {
  pub fn encrypted_size(original_size: usize) -> usize {
    (original_size as f64 / ENCRYPT_CHUNK_SIZE as f64 * DECRYPT_CHUNK_SIZE as f64).floor() as usize
      + HALF_KEY_LENGTH
      + 12
  }

  pub fn original_size(decrypted_size: usize) -> usize {
    ((decrypted_size - 12) as f64 / DECRYPT_CHUNK_SIZE as f64 * ENCRYPT_CHUNK_SIZE as f64).floor()
      as usize
      - HALF_KEY_LENGTH
      - 3
  }
}
