pub enum Status {
  Linux,
  Windows,
  Down,
  Unknown
}

pub struct Machine {
  pub name: String,
  pub status: Status
}
