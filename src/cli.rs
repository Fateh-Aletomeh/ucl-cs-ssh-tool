use clap::Parser;


#[derive(Parser)]
#[command(about)]
pub struct Args {
  /// List all available machines
  #[arg(short, long, exclusive = true)]
  pub list: bool
}
