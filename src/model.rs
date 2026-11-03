#[derive(PartialEq)]
pub enum Status {
  Linux,
  Windows,
  Down
}

pub struct Machine {
  pub name: String,
  pub status: Status
}
