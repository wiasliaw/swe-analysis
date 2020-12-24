extern crate serde_derive;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InputData {
  pub title: String,
  pub company: String,
  pub location: String,
  pub description: String,
}
