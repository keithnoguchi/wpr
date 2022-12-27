use std::io;
use wpr::run;

#[tokio::main]
async fn main() -> io::Result<()> {
    run()?.await
}
