use std::net::TcpListener;

mod ferris_says;
mod server;
mod guess_game;

fn main() {
   ferris_says::ferris_says();

   guess_game::guess_game();

    // Listen for incoming TCP connections on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        server::handle_connection(stream);

        println!("Connection established!");
    }
}
