use crate::log::{progress_value::ProgressValue, progress_viewer::ProgressViewer};

use super::{FileCrypto, FileCryptoErrKind, ENCRYPT_CHUNK_SIZE};
use aes_gcm::{
  aead::{Aead, OsRng},
  AeadCore, Aes256Gcm,
};
use digest::{Key, KeyInit};
use std::{
  fs::{File, OpenOptions},
  io::{Read, Write},
  os::windows::fs::MetadataExt,
  path::Path,
};

impl FileCrypto {
  pub fn encypt<P: AsRef<Path>>(
    src: P,
    dest: P,
    pass: &mut [u8; 32],
    next_line: &mut bool,
    progress_value: &mut ProgressValue,
  ) -> Result<usize, FileCryptoErrKind> {
    let src_path = src.as_ref();
    let dest_path = dest.as_ref();

    let key = Key::<Aes256Gcm>::from_slice(pass);
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let mut input_file = match File::open(src_path) {
      Ok(file) => file,
      Err(_) => return Err(FileCryptoErrKind::FileNotFound),
    };

    let mut output_file = match OpenOptions::new()
      .create(true)
      .write(true)
      .truncate(true)
      .open(dest_path)
    {
      Ok(file) => file,
      Err(_) => return Err(FileCryptoErrKind::FileUnableToCreate),
    };

    if let Err(_) = output_file.write_all(&nonce) {
      return Err(FileCryptoErrKind::FileUnableToWrite);
    };

    let file_size = match input_file.metadata() {
      Ok(md) => md.file_size() as usize,
      Err(_) => return Err(FileCryptoErrKind::FileUnableToRead),
    };
    let mut progress_viewer = ProgressViewer::new();

    let mut buffer = [0u8; ENCRYPT_CHUNK_SIZE];
    let mut written_bytes = nonce.len();

    progress_value.set_size_max(FileCrypto::encrypted_size(file_size));

    loop {
      let bytes_read = match input_file.read(&mut buffer) {
        Ok(bytes) => bytes,
        Err(_) => return Err(FileCryptoErrKind::FileUnableToRead),
      };

      if bytes_read == 0 {
        break;
      }

      let ciphertext = match cipher.encrypt(&nonce, &buffer[..bytes_read]) {
        Ok(enc_text) => enc_text,
        Err(_) => return Err(FileCryptoErrKind::FailedToEncrypt),
      };

      if let Err(_) = output_file.write_all(&ciphertext) {
        return Err(FileCryptoErrKind::FileUnableToWrite);
      };

      written_bytes += ciphertext.len();
      *next_line = true;
      progress_value.set_size_current(written_bytes);
      progress_viewer.update(&progress_value);
    }

    progress_value.set_size_current(written_bytes + 1);
    progress_viewer.update(&progress_value);

    Ok(written_bytes)
  }
}
