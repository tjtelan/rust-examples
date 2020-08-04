pub mod cli;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct RemoteCommand {
    #[structopt(long = "server")]
    pub target_addr: Option<String>,
    pub command: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "server")]
    Server,
    #[structopt(setting = structopt::clap::AppSettings::TrailingVarArg)]
    Run(RemoteCommand),
}

#[derive(StructOpt, Debug)]
#[structopt(name = "remotecli")]
struct ApplicationArguments {
    #[structopt(flatten)]
    pub command: Command,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = ApplicationArguments::from_args();
    //println!("{:?}", opt);

    match opt.command {
        Command::Server => {
            println!("Start the server");
            cli::server::start_server().await?;
        }
        Command::Run(rc) => {
            println!("Run command: '{:?}'", rc.command);
            cli::client::client_run(rc).await?;
        }
    }

    Ok(())
}
