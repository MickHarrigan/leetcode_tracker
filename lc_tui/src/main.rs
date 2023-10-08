mod app;
use app::*;

fn main() -> std::io::Result<()> {
    App::run()
}

#[cfg(test)]
mod test;
