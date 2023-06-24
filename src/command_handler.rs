
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "p", long = "port", value_name = "port-for-startup", default_value = "6379")]
    port: String
}

///
/// Fetches port if passed through command-line arguments or 6379
/// <br>Since: 0.0.1
/// <br>Author: Anurag Ambuj
pub fn get_port() -> String {
    let args = Cli::from_args();
    args.port
}