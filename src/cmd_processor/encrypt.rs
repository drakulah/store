use argon2::Argon2;
use console::style;
use std::path::PathBuf;

use super::CommandProcessor;
use crate::{
  file_crypto::FileCrypto,
  log::{self, progress_value::ProgressValue},
  utils::getch,
  DEFAULT_ENCRYPTED_FILE_EXT,
};

impl CommandProcessor {
  pub fn encrypt(src: &PathBuf, dest: Option<&PathBuf>, force: bool) {
    let maybe_password = match rpassword::prompt_password("Enter password: ") {
      Ok(p) => p,
      Err(_) => return log::error("unable to read the password"),
    };

    let password = maybe_password.as_bytes();
    let password_len = password.len();
    let mut key: Vec<u8> = Vec::new();
    let mut salt = [0u8; 8];

    if password_len < 8 {
      return log::error("password must be at least 8 characters long");
    } else if password_len == 8 {
      salt.copy_from_slice(&password[0..]);
    } else {
      let salt_start = password_len - 8;
      password[..salt_start].clone_into(&mut key);
      salt.copy_from_slice(&password[salt_start..]);
    }

    let mut hashed_password = [0u8; 32];
    let argon2 = Argon2::default();

    if argon2
      .hash_password_into(&key, &salt, &mut hashed_password)
      .is_err()
    {
      return log::error("unable to generate password hash");
    }

    if !src.exists() || !src.is_file() {
      return log::error(&format!(
        "input file not found \"{}\"",
        src.to_string_lossy()
      ));
    }

    let mut default_dest = src.clone();
    let dest_path = match dest {
      Some(p) => p,
      None => {
        let mut filename = match default_dest.file_name() {
          Some(n) => n.to_string_lossy().into_owned(),
          None => {
            return log::error(&format!(
              "input file not found \"{}\"",
              src.to_string_lossy()
            ))
          }
        };

        filename.push_str(DEFAULT_ENCRYPTED_FILE_EXT);
        default_dest.set_file_name(filename);
        &default_dest
      }
    };

    if dest_path.exists() && !force {
      log::warning("output path exists. Overwrite? [Y/n]");
      let inp_ch = getch();
      if inp_ch == 'n' || inp_ch == 'N' {
        return log::abort("encryption process aborted!");
      }
    }

    let mut next_line = false;
    let mut progress_value = ProgressValue::new();

    progress_value.set_prefix("processed");

    match FileCrypto::encypt(
      src,
      dest_path,
      &mut hashed_password,
      &mut next_line,
      &mut progress_value,
    ) {
      Ok(info) => info,
      Err(e) => {
        if next_line {
          println!("");
        }
        return log::error(&e.to_string());
      }
    };

    if next_line {
      println!("");
    }

    log::success(&format!(
      "produced encrypted file \"{}\"",
      style(dest_path.to_string_lossy())
        .blue()
        .bold()
        .underlined()
    ));
  }
}
