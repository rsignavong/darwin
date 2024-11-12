#[macro_use]
extern crate rustler;

use rustler::Term;

mod atoms;
mod gdpr;
mod ulid;
mod uuid;

rustler_export_nifs! {
    "Elixir.Calions.Rustler",
    [
        ("gdpr_decrypt", 2, gdpr::decrypt),
        ("gdpr_encrypt", 1, gdpr::encrypt),
        ("gdpr_encrypt", 2, gdpr::encrypt_with_key),
        ("gdpr_key", 0, gdpr::key),
        ("ulid", 1, ulid::ulid),
        ("uuid_v4", 1, uuid::uuid_v4)
    ],
    None
}
