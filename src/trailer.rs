use std::{collections::HashMap, u32};

#[derive(Debug)]
pub struct Trailer {
    info: Info,
    root: Root,
    size: u32,
}

#[derive(Debug)]
pub struct Info {}

#[derive(Debug)]
pub struct Root {}

impl Trailer {
    pub fn new() -> Self {
        Trailer {
            info: Info {},
            root: Root {},
            size: 0,
        }
    }

    pub fn set_info(&mut self, info: Info) {
        self.info = info;
    }

    pub fn set_root(&mut self, root: Root) {
        self.root = root;
    }

    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }
}
