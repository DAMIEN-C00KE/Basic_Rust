mod server;
mod client;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Please specify 'server' or 'client'");
        return;
    }

    match args[1].as_str() {
        "server" => server::start(),
        "client" => client::start(),
        _ => eprintln!("Unknown command. Use 'server' or 'client'"),
    }
}
