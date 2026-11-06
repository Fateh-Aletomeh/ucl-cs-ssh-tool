use ucl_cs_ssh_tool::run;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let _ = run();
  Ok(())
}
