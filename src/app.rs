use std::{
    io::{self, BufWriter, Write},
    time::{Duration, Instant},
};

use crossterm::{
    event, terminal,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::{
    camera::Camera,
    input::{Action, InputState},
    renderer,
    terminal::display,
};

pub fn run() -> io::Result<()> {
    let mut stdout = BufWriter::new(io::stdout());
    enable_raw_mode()?;
    if let Err(error) = display::setup(&mut stdout) {
        let _ = disable_raw_mode();
        return Err(error);
    }

    let result = run_loop(&mut stdout);
    let cleanup = display::cleanup(&mut stdout);
    let raw_cleanup = disable_raw_mode();
    result.and(cleanup).and(raw_cleanup)
}

fn run_loop<W: Write>(stdout: &mut W) -> io::Result<()> {
    let start = Instant::now();
    let mut camera = Camera::new();
    let mut input = InputState::default();

    loop {
        let frame_start = Instant::now();
        while event::poll(Duration::ZERO)? {
            if let Some(action) = input.handle(event::read()?) {
                match action {
                    Action::Quit => return Ok(()),
                    Action::Orbit { dx, dy } => camera.orbit(dx, dy),
                    Action::Zoom(amount) => camera.zoom(amount),
                }
            }
        }

        let (term_width, term_height) = terminal::size()?;
        if term_width == 0 || term_height < 2 {
            continue;
        }

        let width = term_width as usize * 2;
        let height = (term_height as usize - 1) * 2;
        let time = start.elapsed().as_secs_f32();
        let pixels = renderer::render(width, height, time, &camera);
        let status = format!(
            " BLACK HOLE | drag: orbit | +/- or wheel: zoom | q/Esc: quit | distance: {:.1} ",
            camera.distance
        );
        display::draw(stdout, &pixels, width, &status)?;
        let frame_time = frame_start.elapsed();
        if frame_time < Duration::from_millis(33) {
            std::thread::sleep(Duration::from_millis(33) - frame_time);
        }
    }
}
