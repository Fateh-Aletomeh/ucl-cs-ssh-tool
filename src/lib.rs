pub mod cli;
pub mod config;
pub mod flags;
pub mod model;
pub mod network;
pub mod ssh;

use config::load_config;
use flags::list_machines;
use model::{Machine, Status};
use network::{fetch_html, parse_machines};
use ssh::connect;


pub async fn run(args: cli::Args) -> Result<(), Box<dyn std::error::Error>> {
  // Find all available machines in 1.05 and 1.21

  let url = "https://tsg.cs.ucl.ac.uk/cs-labs/cslabs.html";
  let html = fetch_html(url).await?;

  let (mut lab_105, mut lab_121) = parse_machines(&html);
  lab_105.retain(|m| m.status == Status::Linux);
  lab_121.retain(|m| m.status == Status::Linux);

  if lab_105.len() == 0 && lab_121.len() == 0 {
    println!("All lab machines are down :(");
    return Ok(());
  }

  // Run CLI based on flags

  if args.list {
    list_machines(&lab_105, &lab_121);
  } else {
    let chosen_machine: &Machine;

    if lab_105.len() > 0 {
      chosen_machine = &lab_105[0];
    } else {
      chosen_machine = &lab_121[0];
    }

    let config = load_config();
    connect(&config.username, &chosen_machine.name);
  }

  Ok(())
}
