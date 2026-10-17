
use crossterm::{
    cursor::MoveTo,
    execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    execute!(stdout, Clear(ClearType::All))?;

    let (width, height) = terminal::size()?;

    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;

    for y in 0..height {
        for x in 0..width {
            let px = (x as f32 - cx) * 0.5;
            let py = y as f32 - cy;

            let radius = (px * px + py * py).sqrt();

            // Default: empty space.
            let mut ch = ' ';
            let mut color = Color::Black;

            // Starfield.
            let hash = (x as u32)
                .wrapping_mul(374761393)
                .wrapping_add((y as u32).wrapping_mul(668265263))
                .wrapping_mul(1274126177);

            if hash % 97 == 0 {
                ch = '.';
                color = Color::Rgb {
                    r: 120,
                    g: 100,
                    b: 180,
                };
            }

            // Glowing accretion disk.
            if px.abs() < width as f32 * 0.24
                && py.abs() < 2.0
            {
                ch = '=';
                color = Color::Rgb {
                    r: 255,
                    g: 65,
                    b: 5,
                };
            }

            // Outer glowing ring.
            let ring_radius = height as f32 * 0.20;

            if (radius - ring_radius).abs() < 1.0 {
                ch = 'o';
                color = Color::Rgb {
                    r: 255,
                    g: 100,
                    b: 10,
                };
            }

            // Dark event horizon.
            let horizon = height as f32 * 0.13;

            if radius < horizon {
                ch = ' ';
                color = Color::Black;
            }

            queue!(
                stdout,
                MoveTo(x, y),
                SetForegroundColor(color),
                Print(ch)
            )?;
        }
    }

    queue!(
        stdout,
        SetForegroundColor(Color::Reset),
        MoveTo(0, height.saturating_sub(1)),
        Print("Blackhole v0.1 | Press Ctrl+C to exit")
    )?;

    stdout.flush()?;

    Ok(())
}
