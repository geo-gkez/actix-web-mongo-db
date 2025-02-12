use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Members {
    pub name: String,
    pub surname: String,
    pub email: String,
}
