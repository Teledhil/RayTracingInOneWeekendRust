use std::fs::File;
use std::io::Write;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex,
};

use crate::color::Color;

#[derive(Debug)]
pub struct Buffer {
    width: usize,
    height: usize,
    data: Mutex<Vec<Vec<Color>>>,
    leased_lines: AtomicUsize,
    returned_lines: AtomicUsize,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::with_capacity(height);
        data.resize_with(height, Default::default);
        let data = Mutex::new(data);

        let leased_lines = AtomicUsize::new(0);
        let returned_lines = AtomicUsize::new(0);

        Self {
            width,
            height,
            data,
            leased_lines,
            returned_lines,
        }
    }

    pub fn get_line(&self) -> Vec<Color> {
        self.leased_lines.fetch_add(1, Ordering::Relaxed);
        self.print_status();

        Vec::<Color>::with_capacity(self.width)
    }

    pub fn push_line(&self, height: usize, line: Vec<Color>) {
        self.returned_lines.fetch_add(1, Ordering::Relaxed);
        self.print_status();
        self.data.lock().unwrap()[height] = line;
    }

    pub fn save(&self, filename: &str) -> anyhow::Result<()> {
        print!("Saving buffer... ");
        std::io::stdout().flush().unwrap();

        let mut f = File::create(filename)?;
        write!(&mut f, "P3\n{} {}\n256\n", self.width, self.height)?;

        let data = self.data.lock().unwrap();
        for height in (0..self.height).rev() {
            let line = &data[height];
            for pixel_color in line.iter().take(self.width) {
                write!(f, "{pixel_color}")?;
            }
        }

        println!("Ok");
        Ok(())
    }

    fn print_status(&self) {
        const ESC: char = 27 as char;
        let returned_lines = self.returned_lines.load(Ordering::Relaxed);
        if returned_lines == self.height {
            println!("{ESC}[2K\rDrawing... Ok");
        } else {
            let leased_lines = self.leased_lines.load(Ordering::Relaxed);
            print!(
                "{ESC}[2K\rDrawing... ({}/{}), {} in progress",
                returned_lines,
                self.height,
                leased_lines - returned_lines
            );
            std::io::stdout().flush().unwrap();
        }
    }
}
