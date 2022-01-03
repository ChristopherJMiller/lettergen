
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")] 
pub struct RecipientEntry {
  name: Option<String>,
  street: Option<String>,
  city: Option<String>,
  state: Option<String>,
  #[serde(rename = "Zip Code")] 
  zip_code: Option<String>,
}

impl RecipientEntry {
  pub fn is_valid(&self) -> bool {
    self.name.is_some() && self.street.is_some() && self.city.is_some() && self.state.is_some() && self.zip_code.is_some()
  }
}

impl Into<Recipient> for RecipientEntry {
  fn into(self) -> Recipient {
    Recipient {
      name: self.name.unwrap(),
      street: self.street.unwrap(),
      city: self.city.unwrap(),
      state: self.state.unwrap(),
      zip_code: self.zip_code.unwrap(),
    }
  }
}

#[derive(Debug)]
pub struct Recipient {
  name: String,
  street: String,
  city: String,
  state: String,
  zip_code: String,
}
