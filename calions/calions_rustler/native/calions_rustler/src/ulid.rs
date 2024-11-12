use super::atoms;
use rayon::prelude::*;
use rustler::{Encoder, Env, Error, Term};
use ulid::Generator;

pub fn ulid<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let range: i64 = args[0].decode()?;

    let ulids: Vec<String> = (0..range)
        .into_par_iter()
        .map(|_| Generator::new().generate().map(|ulid| ulid.to_string()))
        .collect::<Result<Vec<String>, _>>()
        .map_err(|_| Error::Atom("error"))?;

    Ok((atoms::ok(), ulids).encode(env))
}
