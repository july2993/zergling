


#[derive(Default, Serialize, Deserialize)]
pub struct UploadResult {
    pub name: String,
    pub size: u32,
    pub error: String,
}
