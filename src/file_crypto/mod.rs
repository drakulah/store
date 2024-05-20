mod decrypt;
mod encrypt;
mod size;

#[derive(Debug)]
pub enum FileCryptoErrKind {
  FileNotFound,
  FileUnableToCreate,
  FileUnableToRead,
  FileUnableToWrite,
  FailedToEncrypt,
  FailedToDecrypt,
}

impl FileCryptoErrKind {
  pub fn to_string(&self) -> String {
    return match self {
      FileCryptoErrKind::FileNotFound => String::from("input file not found"),
      FileCryptoErrKind::FileUnableToCreate => String::from("unable to create new file"),
      FileCryptoErrKind::FileUnableToRead => String::from("unable to read source file"),
      FileCryptoErrKind::FileUnableToWrite => String::from("unable to write contents to new file"),
      FileCryptoErrKind::FailedToEncrypt => String::from("failed to encrypt the file"),
      FileCryptoErrKind::FailedToDecrypt => String::from("failed to decrypt the file"),
    };
  }
}

pub struct FileCrypto {}

pub const KEY_LENGTH: usize = 32;
pub const HALF_KEY_LENGTH: usize = KEY_LENGTH / 2;
pub const ENCRYPT_CHUNK_SIZE: usize = 1024 * 512; // 512 KB
pub const DECRYPT_CHUNK_SIZE: usize = ENCRYPT_CHUNK_SIZE + HALF_KEY_LENGTH;
