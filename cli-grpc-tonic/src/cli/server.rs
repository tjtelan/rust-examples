use std::process::Command;
use std::str;
use tonic::{transport::Server, Request, Response, Status};

// Import the generated rust code into module
pub mod remotecli {
    tonic::include_proto!("remotecli");
}

// Proto generated server traits
use remotecli::remote_cli_server::{RemoteCli, RemoteCliServer};

// Proto message structs
use remotecli::{CommandInput, CommandOutput};

#[derive(Default)]
pub struct Cli {}

#[tonic::async_trait]
impl RemoteCli for Cli {
    async fn shell(
        &self,
        request: Request<CommandInput>,
    ) -> Result<Response<CommandOutput>, Status> {
        let mut command = Command::new(&request.into_inner().command);

        Ok(Response::new(CommandOutput {
            output: String::from_utf8(command.output().expect("failed to execute process").stdout)
                .unwrap(),
        }))
    }
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let cli_server = Cli::default();

    println!("RemoteCliServer listening on {}", addr);

    Server::builder()
        .add_service(RemoteCliServer::new(cli_server))
        .serve(addr)
        .await?;

    Ok(())
}
