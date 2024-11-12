use sha2::{Digest, Sha256};

pub struct Uuid;

impl Uuid {
    pub fn gen_uuid(uuid: &str, parent_uuid: &str, counter: i32) -> String {
        let concatened_uuid = format!("{}{}{}", uuid, parent_uuid, counter);
        Uuid::new_uuid(&concatened_uuid)
    }

    pub fn gen_relationship_uuid(uuid: &str, parents_uuid: &[&str]) -> String {
        let concatened_uuid = format!("{}{}", uuid, parents_uuid.join(""));
        Uuid::new_uuid(&concatened_uuid)
    }

    fn new_uuid(concatened_uuid: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(concatened_uuid.as_bytes());
        let raw = format!("{:02x}", hasher.finalize());
        format!(
            "{}-{}-{}-{}-{}",
            &raw[3..11],
            &raw[11..15],
            &raw[15..19],
            &raw[19..23],
            &raw[23..35]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_gen_uuid() {
        let uuid = "Allium";
        let parent_uuid = "Allium";
        let counter = 0;
        let res = Uuid::gen_uuid(uuid, parent_uuid, counter);
        assert_eq!(res, "a725ffbf-3079-fb75-32f3-3caa522f3411");
    }

    #[test]
    fn simple_gen_relationship_uuid() {
        let uuid = "Allium";
        let parent_uuid = &["Allium", "Allium"];
        let res = Uuid::gen_relationship_uuid(uuid, parent_uuid);
        assert_eq!(res, "4e6edfc3-18df-e99a-eaea-5f21d035f9df");
    }
}
