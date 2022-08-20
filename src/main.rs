//! Source code for Project Name

#![allow(dead_code)]
#![deny(missing_docs)]

use std::fs::read_to_string;

use utility::generation::{
    tile::{RawFile, TileSet},
    wfc::Generator,
};

mod collections;
mod utility;

fn main() {
    let file = read_to_string("data/tiles/test.ron").unwrap();
    let raw: RawFile = ron::from_str(file.as_str()).unwrap();
    let mut set = TileSet::<3>::new(raw.id);
    set.add_all_raws(&raw.tiles);

    let mut gen = Generator::new(3, 3, set.tiles());
    let map = gen.run(false).unwrap();

    println!("{:?}", map);
}
