use shiplift::{
    builder::ContainerFilter, tty::StreamType, ContainerListOptions, ContainerOptions, Docker,
    ExecContainerOptions, PullOptions,
};
use tokio;
use tokio::prelude::{Future, Stream};
use std::io::prelude::*;

use yaml_rust::{YamlEmitter, YamlLoader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new();

    let mut f = std::fs::File::open("config.yml")?;
    let mut file_data = String::new();
    let _ = f.read_to_string(&mut file_data);

    let yaml_data = YamlLoader::load_from_str(&file_data).unwrap();

    // Pull image specified in config
    println!("Pulling image");
    let img = yaml_data[0]["image"].clone().into_string().unwrap();
    let img_pull = docker
        .images()
        .pull(&PullOptions::builder().image(img.clone()).build())
        .for_each(|output| {
            println!("{:?}", output);
            Ok(())
        })
        .map_err(|e| eprintln!("Error: {}", e));
    tokio::run(img_pull);

    // Convert the command from the config to pass into container
    let mut container_command_raw = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut container_command_raw);
        emitter.compact(true);
        emitter.dump(&yaml_data[0]["command"]).unwrap();
    }

    println!(
        "command: {:?}",
        &container_command_raw[4..]
            .split('\n')
            .map(|s| { &s[2..] })
            .collect::<Vec<&str>>()
    );

    let container_command = &container_command_raw[4..]
        .split('\n')
        .map(|s| &s[2..])
        .collect::<Vec<&str>>();

    let container_spec = ContainerOptions::builder(img.as_ref())
        //.auto_remove(true)
        //.name("test-container-name")
        .labels(&[("testkey", "testvalue")].iter().cloned().collect())
        .attach_stdout(true)
        .attach_stderr(true)
        .cmd(vec!["/bin/sh", "-c", "sleep 1h"]) // 1 hour timeout
        .build();

    let new_container = docker
        .containers()
        .create(&container_spec)
        .map(|info| {
            println!("{:?}", info);
            info
        })
        .map_err(|e| {
            eprintln!("Error: {}", e);
            e
        });

    println!("Creating new container");
    let mut container_runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");

    // Wait for the container to be created so we can get its container id
    let r = container_runtime.block_on(new_container);

    let container_id = r.unwrap().id;

    println!("Starting the container");
    let start_container = docker
        .containers()
        .get(&container_id)
        .start()
        .map(|info| {
            println!("{:?}", info);
            info
        })
        .map_err(|e| eprintln!("Error: {}", e));
    tokio::run(start_container);

    // FYI: This might not work until https://github.com/softprops/shiplift/issues/155 is fixed
    println!("Executing commands in the container");
    let options = ExecContainerOptions::builder()
        .cmd(
            [
                "/bin/sh",
                "-c",
                &format!("/bin/sh -c '{}'", container_command.join(";")),
            ]
            .to_vec(),
        )
        .env(vec!["VAR=value"])
        .attach_stdout(true)
        .attach_stderr(true)
        .build();

    let exec_container = docker
        .containers()
        .get(&container_id)
        .exec(&options)
        .for_each(|chunk| {
            match chunk.stream_type {
                StreamType::StdOut => print!("Stdout: {}", chunk.as_string_lossy()),
                StreamType::StdErr => eprintln!("Stderr: {}", chunk.as_string_lossy()),
                StreamType::StdIn => unreachable!(),
            }
            Ok(())
        })
        .map_err(|e| eprintln!("Error: {}", e));

    tokio::run(exec_container);

    println!("Listing the containers made by this example");
    let container_list_opts = ContainerListOptions::builder()
        .all()
        .filter(vec![ContainerFilter::Label(
            "testkey".to_string(),
            "testvalue".to_string(),
        )])
        .build();
    let list_containers = docker
        .containers()
        .list(&container_list_opts)
        .map(|containers| {
            for c in containers {
                println!("container -> {:#?}", c)
            }
        })
        .map_err(|e| println!("Error: {}", e));

    tokio::run(list_containers);

    Ok(())
}
