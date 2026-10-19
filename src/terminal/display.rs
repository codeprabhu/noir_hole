use std::io::{self, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event, execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::renderer::Pixel;

pub fn setup<W: Write>(stdout: &mut W) -> io::Result<()> {
    execute!(
        stdout,
        EnterAlternateScreen,
        event::EnableMouseCapture,
        Hide,
        Clear(ClearType::All)
    )
}

pub fn cleanup<W: Write>(stdout: &mut W) -> io::Result<()> {
    execute!(
        stdout,
        Show,
        event::DisableMouseCapture,
        ResetColor,
        LeaveAlternateScreen
    )
}

pub fn draw<W: Write>(
    stdout: &mut W,
    pixels: &[Pixel],
    width: usize,
    status: &str,
) -> io::Result<()> {
    let (term_width, term_height) = terminal::size()?;
    let rows = term_height.saturating_sub(1) as usize;
    let draw_width = (width / 2).min(term_width as usize);

    for row in 0..rows {
        queue!(stdout, MoveTo(0, row as u16))?;
        for cell_x in 0..draw_width {
            let x = cell_x * 2;
            let samples = [
                pixels[row * 2 * width + x],
                pixels[row * 2 * width + x + 1],
                pixels[(row * 2 + 1) * width + x],
                pixels[(row * 2 + 1) * width + x + 1],
            ];
            let (mask, foreground, background) = encode_cell(samples);
            let glyph = match mask {
                0 => ' ',
                1 => '▘',
                2 => '▝',
                3 => '▀',
                4 => '▖',
                5 => '▌',
                6 => '▞',
                7 => '▛',
                8 => '▗',
                9 => '▚',
                10 => '▐',
                11 => '▜',
                12 => '▄',
                13 => '▙',
                14 => '▟',
                _ => '█',
            };
            queue!(
                stdout,
                SetForegroundColor(Color::Rgb {
                    r: foreground.r,
                    g: foreground.g,
                    b: foreground.b
                }),
                SetBackgroundColor(Color::Rgb {
                    r: background.r,
                    g: background.g,
                    b: background.b
                }),
                Print(glyph)
            )?;
        }
    }

    queue!(
        stdout,
        ResetColor,
        MoveTo(0, term_height.saturating_sub(1)),
        Print(status)
    )?;
    stdout.flush()
}

fn encode_cell(samples: [Pixel; 4]) -> (u8, Pixel, Pixel) {
    let luminance = samples
        .map(|pixel| 0.2126 * pixel.r as f32 + 0.7152 * pixel.g as f32 + 0.0722 * pixel.b as f32);
    let threshold = (luminance.iter().sum::<f32>() / 4.0).max(8.0);
    let mut mask = 0u8;
    for (index, value) in luminance.into_iter().enumerate() {
        if value >= threshold {
            mask |= 1 << index;
        }
    }

    let mut foreground = [0u32; 3];
    let mut background = [0u32; 3];
    let mut foreground_count = 0u32;
    let mut background_count = 0u32;
    for (index, pixel) in samples.into_iter().enumerate() {
        let target = if mask & (1 << index) != 0 {
            foreground_count += 1;
            &mut foreground
        } else {
            background_count += 1;
            &mut background
        };
        target[0] += pixel.r as u32;
        target[1] += pixel.g as u32;
        target[2] += pixel.b as u32;
    }

    let average = |color: [u32; 3], count: u32| -> Pixel {
        if count == 0 {
            return Pixel::black();
        }
        Pixel::new(
            (color[0] / count) as u8,
            (color[1] / count) as u8,
            (color[2] / count) as u8,
        )
    };
    (
        mask,
        average(foreground, foreground_count),
        average(background, background_count),
    )
}
