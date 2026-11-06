use std::process::Command; 


pub fn connect(username: &str, machine_name: &str) {
  println!("Connecting to {}...", machine_name);
  
  let _ = Command::new("ssh")
    .args([
      "-l", username,
      "-J", &format!("{}@knuckles.cs.ucl.ac.uk", username),
      &format!("{}-l.cs.ucl.ac.uk", machine_name),
    ])
    .status()
    .expect("Failed to run ssh command");
}
