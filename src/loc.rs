use codespan::{FileId, Span};

#[derive(Debug, Copy, Clone)]
pub struct Locatable<T> {
    pub data: T,
    pub loc: Span,
}

impl<T> Locatable<T> {
    pub fn new(data: T, loc: impl Into<Span>) -> Self {
        Self {
            data,
            loc: loc.into(),
        }
    }

    pub fn map<O, F: FnOnce(T) -> O>(self, mapper: F) -> Locatable<O> {
        Locatable {
            data: mapper(self.data),
            loc: self.loc,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Location {
    pub file: FileId,
    pub loc: Span,
}

#[derive(Debug, Copy, Clone)]
pub struct FileLocatable<T> {
    pub data: T,
    pub loc: Location,
}

impl<T> FileLocatable<T> {
    pub fn new(data: T, loc: Location) -> Self {
        Self {
            data,
            loc: loc.into(),
        }
    }

    pub fn map<O, F: FnOnce(T) -> O>(self, mapper: F) -> FileLocatable<O> {
        FileLocatable {
            data: mapper(self.data),
            loc: self.loc,
        }
    }
}
