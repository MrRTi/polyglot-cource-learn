use std::path::PathBuf;
use anyhow::{anyhow, Context, Result};

fn error_me(throw: bool) -> Result<(), usize> {
    if throw {
        return Err(anyhow!("this should never be true"));
    }

    std::fs::read(PathBuf::from("/foo")).context("Add context to error");

    return Ok(());
}

fn main() -> Result<(), usize> {
    error_me(false)?;

    if error_me(true).is_ok() {
        // smth
    }

    return Ok(());
}
