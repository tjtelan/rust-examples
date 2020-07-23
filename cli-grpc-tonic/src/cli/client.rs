// Import the generated rust code into module
pub mod remotecli {
    tonic::include_proto!("remotecli");
}

// Proto generated client traits
use remotecli::remote_cli_client::RemoteCliClient;

// Proto message structs
use remotecli::CommandInput;

use crate::RemoteCommand;

pub async fn client_run(rc: RemoteCommand) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to server
    // Use server addr if given, otherwise use default
    let mut client = match rc.target_addr {
        Some(addr) => RemoteCliClient::connect(addr).await?,
        None => RemoteCliClient::connect("http://[::1]:50051").await?,
    };

    let request = tonic::Request::new(CommandInput {
        command: rc.command[0].clone().into(),
        args: rc.command[1..].to_vec(),
    });

    let response = client.shell(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
