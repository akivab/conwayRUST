use std::{net::TcpStream, time::Duration};
use std::{net::TcpListener, thread, usize};
use std::{io::Write};
use rand::prelude::*;
use thread::Thread;
use web_server::threadpool::ThreadPool;

struct ConwayGameOfLife {
    cells: [[bool; 80]; 40]
}

impl std::fmt::Display for ConwayGameOfLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        for row in self.cells.iter() {
            for cell in row {
                match write!(f, "{}", if *cell { "*" } else { " " }) {
                    Err(q) => return Err(q),
                    _ => {}
                }
            }
            match write!(f, "\n") {
                Err(q) => return Err(q),
                _ => {}
            }
        }
        return Ok(());
    }
}

impl ConwayGameOfLife {
    fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        self.cells.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                *cell = rng.gen_bool(0.5);
            });
        })
    }

    fn step(&mut self) {
        let mut updated = self.cells.clone();
        let rows = self.cells.len();
        let cols = self.cells[0].len();
        for i in 0..rows {
            for j in 0..cols {
                let imalive = self.cells[i][j];
                let mut neighbor_count = 0;
                for k in -1..=1 as i32 {
                    for l in -1..=1 as i32 {
                        if k == 0 && l == 0 { continue; }
                        let mut x = i as i32 + k;
                        let mut y = j as i32 + l;
                        if x < 0 {
                            x = (rows - 1) as i32;
                        }
                        if y < 0 {
                            y = (cols - 1) as i32;
                        }
                        if x >= rows as i32 {
                            x = 0 as i32;
                        }
                        if y >= cols as i32 {
                            y = 0 as i32;
                        }
                        neighbor_count += if self.cells[x as usize][y as usize] { 1 } else { 0 }
                    }
                }
                updated[i][j] = if imalive { neighbor_count >= 2 && neighbor_count <= 3 } else { neighbor_count == 3 };
            }
        }
        self.cells = updated;
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut game_of_life = ConwayGameOfLife { cells: [[false; 80]; 40] };
    game_of_life.generate();

    loop {
        thread::sleep(Duration::from_millis(10));
        stream.write(format!("{esc}c", esc = 27 as char).as_bytes())?;
        game_of_life.step();
        stream.write(format!("{}", game_of_life).as_bytes())?;
        stream.flush()?;
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            match handle_connection(stream) {
                Err(q) => println!("saw err {}", q),
                _ => { }
            }
        });
    }
}