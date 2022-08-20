use rand::{distributions::WeightedIndex, thread_rng, Rng};

use crate::collections::grid::{Grid, Idx};

use super::tile::{Side, Tile, TransformedTile};

/// Contains errors that can be encountered while working with the generator
#[derive(Debug)]
pub enum Error {
    EmptySet,
    InvalidIndex,
    InvalidWeight,
    MissingSet,
    MissingTile,
}

/// Implements the wave function collapse algorithm
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Generator<const P: usize>(Grid<Vec<TransformedTile<P>>>);

impl<const P: usize> Generator<P> {
    /// Creates a new generator
    pub fn new(width: usize, height: usize, tiles: &[TransformedTile<P>]) -> Self {
        let mut tiles = tiles.to_vec();
        tiles.dedup();
        Self(Grid::new_with(width, height, tiles))
    }

    /// Returns a reference to the generator's grid
    pub const fn grid(&self) -> &Grid<Vec<TransformedTile<P>>> {
        &self.0
    }
    /// Returns a mutable reference to the generator's grid
    pub fn grid_mut(&mut self) -> &mut Grid<Vec<TransformedTile<P>>> {
        &mut self.0
    }

    /// Returns the grid's total number of possible tiles
    pub fn entropy(&self) -> usize {
        self.grid()
            .iter()
            .map(|o| o.as_ref().unwrap_or(&Vec::new()).len())
            .reduce(|a, b| a + b)
            .unwrap_or_default()
    }
    /// Returns `true` if the grid has only one possible state
    pub fn is_collapsed(&self) -> bool {
        self.grid()
            .iter()
            .all(|o| o.as_ref().map_or(false, |v| v.len() == 1))
    }
    /// Returns `true` if the grid is entirely out of possible states, which should never happen
    pub fn is_empty(&self) -> bool {
        self.grid()
            .iter()
            .all(|o| o.as_ref().map_or(false, Vec::is_empty))
    }
    /// Returns `true` if any space in the grid is out of possible states, which should never happen
    pub fn is_any_empty(&self) -> bool {
        self.grid()
            .iter()
            .any(|o| o.as_ref().map_or(false, Vec::is_empty))
    }

    /// Returns a list of possible grid indices that are directly adjacent to the provided coordinates
    pub fn adjacent(&self, (x, y): Idx) -> Vec<Idx> {
        [
            (x.saturating_sub(1), y),
            (x.saturating_add(1), y),
            (x, y.saturating_sub(1)),
            (x, y.saturating_add(1)),
        ]
        .into_iter()
        .filter(|c| *c != (x, y))
        .filter(|(x, _)| *x < self.grid().width())
        .filter(|(_, y)| *y < self.grid().height())
        .collect::<Vec<_>>()
    }
    /// Returns a grid index for the tile set with the lowest number of possible tiles
    pub fn next_index(&self) -> Result<Idx, Error> {
        let tiles = self
            .grid()
            .enumerate()
            .filter(|(_, o)| o.as_ref().map_or(false, |v| v.len() > 1));

        let entropy = tiles
            .clone()
            .min_by_key(|(_, o)| o.as_ref().map_or(usize::MAX, Vec::len))
            .ok_or(Error::MissingSet)?
            .1
            .as_ref()
            .ok_or(Error::MissingSet)?
            .len();

        let tiles = tiles
            .filter(|(_, o)| o.as_ref().map_or(false, |v| v.len() == entropy))
            .collect::<Vec<_>>();

        if tiles.is_empty() {
            Err(Error::MissingSet)
        } else {
            let index = thread_rng().gen_range(0..tiles.len());
            let (coords, _) = tiles.get(index).ok_or(Error::MissingSet)?;
            Ok(*coords)
        }
    }
    /// Collapses the tile set at the provided coordinates into a random possible tile
    pub fn collapse(&mut self, position: Idx) -> Result<(), Error> {
        let set = self.grid_mut().get_mut(position).ok_or(Error::MissingSet)?;

        if set.is_empty() {
            Err(Error::EmptySet)
        } else {
            let weights = set.iter().map(|t| t.tile().weight());
            let weights = WeightedIndex::new(weights).map_err(|_| Error::InvalidWeight)?;
            let index = thread_rng().sample(weights);

            set.swap(0, index);
            set.drain(1..);
            Ok(())
        }
    }
    /// Updates all tile sets surrounding the provided position until all affected sets have been updated
    pub fn propogate(&mut self, position: Idx) -> Result<usize, Error> {
        let mut stack = vec![position];
        let mut loops = 0_usize;

        while let Some(index) = stack.pop() {
            let set = self.grid().get(index).ok_or(Error::MissingSet)?.clone();

            for adjacent in self.adjacent(index) {
                let side = Side::relative(index, adjacent);
                let other = self.grid_mut().get_mut(adjacent).ok_or(Error::MissingSet)?;

                for tile in other.clone() {
                    if !set
                        .iter()
                        .any(|t| t.transformed().connects_to(&tile.transformed(), side))
                    {
                        other.retain(|t| t != &tile);

                        if !stack.contains(&adjacent) {
                            stack.push(adjacent);
                        }
                    }
                }
            }

            loops += 1;
        }

        Ok(loops)
    }

    pub fn run(&mut self, silent: bool) -> Result<Grid<Tile<P>>, Error> {
        if !silent {
            println!(
                "Generating... {}x{} ({})",
                self.grid().width(),
                self.grid().height(),
                self.grid().capacity()
            );
        }

        let mut cycles = 0_usize;
        let mut props = 0_usize;

        while !self.is_collapsed() {
            if !silent {
                println!("\tC: {cycles}\tP: {props}\tE: {}", self.entropy());
            }

            let index = self.next_index()?;

            self.collapse(index)?;
            props += self.propogate(index)?;
            cycles += 1;

            if self.is_any_empty() {
                return Err(Error::EmptySet);
            }
        }

        if !silent {
            println!("Generation completed.");
        }

        Ok(self.grid().clone().map_some(|s| s[0].transformed()))
    }
}
