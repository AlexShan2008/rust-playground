mod ferris_says;
mod server;
mod guess_game;

fn main() {
   server::start_server();

   ferris_says::ferris_says();

   guess_game::guess_game();
}
