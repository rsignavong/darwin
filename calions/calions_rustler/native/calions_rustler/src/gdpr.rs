use super::atoms;
use gdpr::{GdprKey, GdprValue};
use rustler::{Encoder, Env, Error, Term};
use std::convert::TryFrom;

pub fn decrypt<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let value: &str = args[0].decode()?;
    let key: &str = args[1].decode()?;

    let gdpr_value = GdprValue::try_from(value).map_err(|_| Error::Atom("error_value"))?;
    let gdpr_key = GdprKey::try_from(key).map_err(|_| Error::Atom("error_key"))?;

    let text = gdpr_value
        .decrypt(&gdpr_key)
        .map_err(|_| Error::Atom("error_decrypt"))?;

    Ok((atoms::ok(), text).encode(env))
}

pub fn encrypt<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let value: &str = args[0].decode()?;

    let (gdpr_key, gdpr_value) =
        GdprValue::encrypt(value).map_err(|_| Error::Atom("error_encrypt"))?;

    Ok((
        atoms::ok(),
        vec![gdpr_key.to_string(), gdpr_value.to_string()],
    )
        .encode(env))
}

pub fn encrypt_with_key<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let value: &str = args[0].decode()?;
    let key: &str = args[1].decode()?;

    let gdpr_key = GdprKey::try_from(key).map_err(|_| Error::Atom("error_key"))?;
    let gdpr_value = GdprValue::encrypt_with_key(value, &gdpr_key)
        .map_err(|_| Error::Atom("error_encrypt_with_key"))?;

    Ok((atoms::ok(), gdpr_value.to_string()).encode(env))
}

pub fn key<'a>(env: Env<'a>, _args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let gdpr_key = GdprKey::gen().map_err(|_| Error::Atom("error_key"))?;

    Ok((atoms::ok(), gdpr_key.to_string()).encode(env))
}
