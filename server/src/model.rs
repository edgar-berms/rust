use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    RegisterTeam(RegisterTeam),
    JoinTeam(JoinTeam),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterTeam {
    pub team_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinTeam {
    pub access_code: String,
    pub player_name: String,
}
