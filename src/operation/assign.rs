
// #[macro_use]
// extern crate serde_derive;





#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AssignResult {
    pub fid: String,
    pub url: String,
    pub publicUrl: String,
    pub count: u64,
    pub error: String,
}
