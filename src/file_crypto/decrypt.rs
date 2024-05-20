use crate::log::{progress_value::ProgressValue, progress_viewer::ProgressViewer};

use super::{FileCrypto, FileCryptoErrKind, DECRYPT_CHUNK_SIZE};
use aes_gcm::{aead::Aead, Aes256Gcm};
use digest::{generic_array::GenericArray, Key, KeyInit};
use std::{
  fs::{File, OpenOptions},
  io::{Read, Write},
  os::windows::fs::MetadataExt,
  path::Path,
};

impl FileCrypto {
  pub fn decypt<P: AsRef<Path>>(
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

    let mut nonce_slice = [0u8; 12];

    match input_file.read(&mut nonce_slice) {
      Ok(bytes) => {
        if bytes == 0 {
          return Ok(0);
        }
      }
      Err(_) => return Err(FileCryptoErrKind::FileUnableToRead),
    };

    let file_size = match input_file.metadata() {
      Ok(md) => (md.file_size() as usize) - 12_usize,
      Err(_) => return Err(FileCryptoErrKind::FileUnableToRead),
    };
    let mut progress_viewer = ProgressViewer::new();

    let nonce = GenericArray::from_slice(&nonce_slice);
    let mut buffer = [0u8; DECRYPT_CHUNK_SIZE];
    let mut written_bytes = 0;

    progress_value.set_size_max(FileCrypto::original_size(file_size));

    loop {
      let bytes_read = match input_file.read(&mut buffer) {
        Ok(bytes) => bytes,
        Err(_) => return Err(FileCryptoErrKind::FileUnableToRead),
      };

      if bytes_read == 0 {
        break;
      }

      let plaintext = match cipher.decrypt(&nonce, &buffer[..bytes_read]) {
        Ok(enc_text) => enc_text,
        Err(_) => return Err(FileCryptoErrKind::FailedToDecrypt),
      };

      if let Err(_) = output_file.write_all(&plaintext) {
        return Err(FileCryptoErrKind::FileUnableToWrite);
      };

      written_bytes += plaintext.len();
      *next_line = true;
      progress_value.set_size_current(written_bytes);
      progress_viewer.update(&progress_value);
    }

    Ok(written_bytes)
  }
}
