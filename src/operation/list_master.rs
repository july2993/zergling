#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ClusterStatusResult {
    pub IsLeader: bool,
    pub Leader: String,
    pub Peers: Vec<String>,
}



// pub fn list_masters(server: &str) -> Result<(String, Vec<String>)> {
//     let url = format!("http://{}/cluster/")
// }
