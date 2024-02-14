fn error_me(throw: bool) -> Result<(), usize> {
    if throw {
        return Err(69);
    }

    return Ok(());
}

fn main() -> Result<(), usize> {
    error_me(false)?;

    if error_me(true).is_ok() {
        // smth
    }

    return Ok(());
}
