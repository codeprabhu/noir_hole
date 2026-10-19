mod app;
mod camera;
mod input;
mod math;
mod renderer;
mod terminal;

fn main() -> std::io::Result<()> {
    app::run()
}
