use std::path::Path;

/// Represent a location in a file
/// The line column is 0
/// The first column is 0
#[derive(Clone, Default)]
pub struct Location {
    line: usize,
    col: usize,
}

pub struct FileLocation<'filepath> {
    filepath: Option<&'filepath Path>,
    location: Location,
}

impl Location {
    pub fn new<T: Into<usize>, U: Into<usize>>(line: T, col: U) -> Self {
        Self { line: line.into(), col: col.into() }
    }

    pub fn human(&self) -> (usize, usize) {
        (self.line.saturating_add(1), self.col.saturating_add(1))
    }

    pub fn incr_col(&mut self) {
        self.col = self.col.saturating_add(1);
    }

    pub fn incr_line(&mut self) {
        self.line = self.line.saturating_add(1);
        self.col = 0;
    }
}

impl<T: Into<usize>, U: Into<usize>> From<(T, U)> for Location {
    fn from(value: (T, U)) -> Self {
        Self { line: value.0.into(), col: value.1.into() }
    }
}

impl<'filepath> FileLocation<'filepath> {
    pub fn new<'b: 'filepath>(filepath: &'b Path, location: Location) -> Self {
        Self { filepath: Some(filepath), location }
    }

    pub fn incr_col(&mut self) {
        self.location.incr_col();
    }

    pub fn incr_line(&mut self) {
        self.location.incr_line();
    }
}

impl<'filepath> From<&'filepath Path> for FileLocation<'filepath> {
    fn from(filepath: &'filepath Path) -> Self {
        Self { filepath: Some(filepath), location: Location::default() }
    }
}
