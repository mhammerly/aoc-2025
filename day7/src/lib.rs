use std::collections::HashSet;
use std::io::{BufRead, Lines};

#[derive(thiserror::Error, Debug)]
pub enum ManifoldError {
    #[error("no manifold!")]
    MissingManifold,

    #[error("missing beam start")]
    MissingStart,

    #[error("unknown tachyon obstruction: {0}")]
    UnknownObstruction(char),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[derive(Default)]
pub struct TachyonManifold {
    pub beams: HashSet<usize>,
    pub splits: u16,
}

impl TachyonManifold {
    fn tick(&mut self, line: &str) -> Result<(), ManifoldError> {
        let splitters: Vec<_> = line
            .as_bytes()
            .iter()
            .enumerate()
            .filter_map(|(i, c)| (*c == b'^').then_some(i))
            .collect();

        // Debug output - fill in all beams that aren't interrupted by a splitter
        let mut line_debug = line.to_string();
        for beam in self.beams.iter() {
            if splitters
                .iter()
                .find(|splitter| *splitter == beam)
                .is_none()
            {
                unsafe {
                    line_debug.as_bytes_mut()[*beam] = b'|';
                }
            }
        }

        for splitter in splitters.iter() {
            if self.beams.contains(splitter) {
                self.beams.remove(splitter);
                if *splitter > 0 {
                    self.beams.insert(splitter - 1);
                    unsafe {
                        // Debug output - fill in a beam that was split
                        line_debug.as_bytes_mut()[splitter - 1] = b'|';
                    }
                }
                if *splitter < line.len() - 1 {
                    self.beams.insert(splitter + 1);
                    unsafe {
                        // Debug output - fill in a beam that was split
                        line_debug.as_bytes_mut()[splitter + 1] = b'|';
                    }
                }
                self.splits += 1;
            }
        }

        tracing::debug!("tick: {}", line_debug);

        Ok(())
    }

    pub fn run<B: BufRead>(mut lines: Lines<B>) -> Result<TachyonManifold, ManifoldError> {
        let mut manifold = TachyonManifold::default();

        let first_line = lines.next().ok_or(ManifoldError::MissingManifold)??;
        tracing::debug!("tick: {}", first_line);
        let starting_beam = first_line
            .bytes()
            .position(|c| c == b'S')
            .ok_or(ManifoldError::MissingStart)?;

        manifold.beams.insert(starting_beam);
        for line in lines {
            manifold.tick(&line?)?;
        }
        Ok(manifold)
    }
}
