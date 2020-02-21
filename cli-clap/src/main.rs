#[macro_use]
extern crate clap;
use clap::App;

fn main() {

    let yml = load_yaml!("cli.yml");
    let m = App::from_yaml(yml).get_matches();

    // Because the example 17_yaml.yml is rather large we'll just look a single arg so you can
    // see that it works...
    if let Some(mode) = m.value_of("mode") {
        match mode {
            "vi" => println!("You are using vi"),
            "emacs" => println!("You are using emacs..."),
            _      => unreachable!()
        }
    } else {
        println!("--mode <MODE> wasn't used...");
    }
}
