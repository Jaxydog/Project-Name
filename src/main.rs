//! Source code for Project Name

#![allow(dead_code)]
#![deny(missing_docs)]

use utility::{
    grid::Grid,
    wfc::{Generator, TileGenerator},
};

use crate::utility::wfc::Rotation;

mod utility;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut gen = TileGenerator::new();

    for texture in textures() {
        let grid = Grid::from(texture);
        gen.generate(&grid, [(0, 0, 0); 4], 1);
    }

    let mut wfc = Generator::new(gen.tiles().clone(), 10, 10);

    if let Ok(grid) = wfc.run() {
        let tiles = textures()
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|v| v.into_iter().map(color).collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            })
            .map(Grid::from)
            .collect::<Vec<_>>();

        let grid = grid
            .map(|t| (*t, tiles[t.id()].clone()))
            .map(|(tile, grid)| {
                let mut grid = grid.clone();

                match tile.rotation() {
                    Rotation::None => (),
                    Rotation::Once => grid.rotate_right(),
                    Rotation::Twice => grid.rotate_twice(),
                    Rotation::Thrice => grid.rotate_left(),
                }
                if tile.x_flipped() {
                    grid.flip_x();
                }
                if tile.y_flipped() {
                    grid.flip_y();
                }

                grid
            })
            .map(|g| {
                g.as_vec()
                    .clone()
                    .into_iter()
                    .map(|v| v.join(""))
                    .collect::<Vec<_>>()
            });

        let mut final_grid = String::new();

        for y in 0..grid.height() {
            for r in 0..5 {
                for x in 0..grid.width() {
                    final_grid.push_str(grid.as_vec()[y][x][r].as_str());
                }
                final_grid.push('\n');
            }
        }

        println!("{}", final_grid);
    }
}

fn color(n: usize) -> &'static str {
    match n {
        0 => "⬜",
        1 => "⬛",
        2 => "🟥",
        _ => "❓",
    }
}

fn textures() -> Vec<Vec<Vec<usize>>> {
    vec![
        vec![
            vec![1, 1, 0, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 2, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1],
        ],
        vec![
            vec![1, 1, 0, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 2, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 0, 1, 1],
        ],
        vec![
            vec![1, 1, 0, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![0, 0, 2, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 0, 1, 1],
        ],
        vec![
            vec![1, 1, 0, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![0, 0, 2, 0, 0],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 0, 1, 1],
        ],
        vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
        ],
        vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ],
        vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
        ],
        vec![
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
        ],
        vec![
            vec![1, 1, 0, 1, 1],
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
        ],
        vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
        ],
        vec![
            vec![1, 1, 0, 1, 1],
            vec![1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
        ],
        vec![
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
        ],
    ]
}
