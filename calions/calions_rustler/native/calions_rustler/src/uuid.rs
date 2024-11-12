use rayon::prelude::*;
use rustler::{Encoder, Env, Error, Term};
use uuid::Uuid;

pub fn uuid_v4<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let range: i64 = args[0].decode()?;

    let uuids: Vec<String> = (0..range)
        .into_par_iter()
        .map(|_| Uuid::new_v4().to_string())
        .collect();

    Ok(uuids.encode(env))
}
