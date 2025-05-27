use ::serde::Deserialize;

#[derive(Deserialize)]
pub struct Pair {
    pub key: String,
    pub value:String,
}