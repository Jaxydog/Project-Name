use std::array::IntoIter;

use rand::{distributions::WeightedIndex, thread_rng, Rng};
use serde::{Deserialize, Serialize};

use super::grid::Grid;

/// Contains errors that can be encountered while working with the generator
#[derive(Debug)]
pub enum Error {
    EmptySet,
    InvalidWeight,
    MissingSet,
    MissingTile,
    NoValidSet,
    SizeMismatch,
}

/// Represents one side of a tile
#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    Left = 0,
    Top = 1,
    Right = 2,
    Bottom = 3,
}

impl Side {
    /// Returns the side opposite to this one
    pub const fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Top => Self::Bottom,
            Self::Right => Self::Left,
            Self::Bottom => Self::Top,
        }
    }
    /// Returns `Self::Left` or `Self::Right` depending on whether start is greater than end
    pub const fn relative_x(start: usize, end: usize) -> Self {
        if start > end {
            Self::Left
        } else {
            Self::Right
        }
    }
    /// Returns `Self::Top` or `Self::Bottom` depending on whether start is greater than end
    pub const fn relative_y(start: usize, end: usize) -> Self {
        if start > end {
            Self::Top
        } else {
            Self::Bottom
        }
    }
    /// Returns a side depending on the direction of movement from one position to the other
    pub const fn relative(start: (usize, usize), end: (usize, usize)) -> Self {
        if start.0 == end.0 {
            Self::relative_y(start.1, end.1)
        } else {
            Self::relative_x(start.0, end.0)
        }
    }
    /// Returns an iterator over all possible sides
    pub fn iter() -> IntoIter<Self, 4> {
        [Self::Left, Self::Top, Self::Right, Self::Bottom].into_iter()
    }
}

impl From<Side> for usize {
    fn from(side: Side) -> Self {
        side as Self
    }
}

/// Represents all possible rotations of a tile
#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rotation {
    D0 = 0,
    D90 = 1,
    D180 = 2,
    D270 = 3,
}

impl Rotation {
    /// Returns an iterator over all possible rotations
    pub fn iter() -> IntoIter<Self, 4> {
        [Self::D0, Self::D90, Self::D180, Self::D270].into_iter()
    }
}

impl From<Rotation> for usize {
    fn from(rotation: Rotation) -> Self {
        rotation as Self
    }
}

/// Stores data for one side of a tile, used for ensuring that two tiles can fit together
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Socket<const P: usize>([usize; P]);

impl<const P: usize> Socket<P> {
    /// Creates a new socket
    pub const fn new(nodes: [usize; P]) -> Self {
        Self(nodes)
    }

    /// Returns a reference to the socket's nodes
    pub const fn nodes(&self) -> &[usize; P] {
        &self.0
    }

    /// Returns a flipped copy of the socket
    pub fn flipped(&self) -> Self {
        let mut nodes = *self.nodes();
        nodes.reverse();
        Self(nodes)
    }
    /// Returns `true` if the socket is symmetrical
    pub fn symmetrical(&self) -> bool {
        self == &self.flipped()
    }
}

impl<const P: usize, T: Into<usize>> From<[T; P]> for Socket<P> {
    fn from(array: [T; P]) -> Self {
        Self::new(array.map(std::convert::Into::into))
    }
}

impl<const P: usize, T: Into<usize>> From<Vec<T>> for Socket<P> {
    fn from(value: Vec<T>) -> Self {
        let mut array = [0; P];

        for (index, number) in value.into_iter().enumerate().take(P) {
            array[index] = number.into();
        }

        array.into()
    }
}

/// Represents one possible state within the generator's grid
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tile<const P: usize> {
    id: usize,
    layer: usize,
    weight: usize,
    sockets: [Socket<P>; 4],
    rotation: Rotation,
    flip: (bool, bool),
}

impl<const P: usize> Tile<P> {
    /// Creates a new tile
    pub const fn new(
        id: usize,
        layer: usize,
        weight: usize,
        sockets: [Socket<P>; 4],
        rotation: Rotation,
        flip: (bool, bool),
    ) -> Self {
        Self {
            id,
            layer,
            weight,
            sockets,
            rotation,
            flip,
        }
    }

    /// Returns the tile's identifier
    pub const fn id(&self) -> usize {
        self.id
    }
    /// Returns the tile's layer identifier
    pub const fn layer(&self) -> usize {
        self.layer
    }
    /// Returns the tile's weight
    pub const fn weight(&self) -> usize {
        self.weight
    }
    /// Returns the tile's sockets
    pub const fn sockets(&self) -> [Socket<P>; 4] {
        self.sockets
    }
    /// Returns the tile's socket on the given side
    pub fn socket(&self, side: Side) -> Socket<P> {
        self.sockets[usize::from(side)]
    }
    /// Returns the tile's rotation
    pub const fn rotation(&self) -> Rotation {
        self.rotation
    }
    /// Returns `true` if the tile is flipped horizontally
    pub const fn x_flipped(&self) -> bool {
        self.flip.0
    }
    /// Returns `true` if the tile is flipped vertically
    pub const fn y_flipped(&self) -> bool {
        self.flip.1
    }

    /// Returns a copy of the tile that has been rotated if necessary
    pub fn rotated(&self) -> Self {
        let mut rotated = *self;
        rotated.sockets.rotate_right(self.rotation().into());
        rotated.rotation = Rotation::D0;
        rotated
    }
    /// Returns a copy of the tile that has been flipped if necessary
    pub fn flipped(&self) -> Self {
        let mut flipped = *self;

        if flipped.x_flipped() {
            flipped.sockets.swap(Side::Left.into(), Side::Right.into());
            flipped.sockets[usize::from(Side::Top)] = flipped.socket(Side::Top).flipped();
            flipped.sockets[usize::from(Side::Bottom)] = flipped.socket(Side::Bottom).flipped();
            flipped.flip.0 = false;
        }
        if flipped.y_flipped() {
            flipped.sockets.swap(Side::Top.into(), Side::Bottom.into());
            flipped.sockets[usize::from(Side::Left)] = flipped.socket(Side::Left).flipped();
            flipped.sockets[usize::from(Side::Right)] = flipped.socket(Side::Right).flipped();
            flipped.flip.1 = false;
        }

        flipped
    }
    /// Returns a copy of the tile that has been transformed if necessary
    pub fn transformed(&self) -> Self {
        self.rotated().flipped()
    }
    /// Returns `true` if the provided tile is compatible with the current tile on the given side
    pub fn connects(&self, tile: &Self, side: Side) -> bool {
        let facing = side.opposite();
        let this = self.transformed().socket(side);
        let other = tile.transformed().socket(facing);

        self.layer() == tile.layer() && this == other
    }
}

/// Enables parsing from the implementing value into a list of tiles
pub trait ToTiles<const P: usize> {
    /// Returns a new socket for the provided side, parsed from the implementing value
    fn to_socket(&self, side: Side) -> Socket<P>;

    /// Returns all possible tiles that can be parsed from the implementing value
    fn to_tiles(&self, id: usize, layer: usize, weight: usize) -> Vec<Tile<P>> {
        let mut tiles = Vec::new();
        let mut sockets = [Socket::new([0; P]); 4];

        for (index, socket) in Side::iter().map(|s| self.to_socket(s)).enumerate() {
            sockets[index] = socket;
        }

        for rotation in Rotation::iter() {
            for flips in 0..4 {
                let flip = match flips {
                    1 => (true, false),
                    2 => (false, true),
                    3 => (true, true),
                    _ => (false, false),
                };

                tiles.push(Tile::new(id, layer, weight, sockets, rotation, flip));
            }
        }

        tiles.dedup();
        tiles
    }
}

impl<const P: usize, T: Clone + Into<usize>> ToTiles<P> for Grid<T> {
    fn to_socket(&self, side: Side) -> Socket<P> {
        match side {
            Side::Left => {
                let mut swapped = self.clone();
                swapped.transpose();
                swapped[0].clone().into()
            }
            Side::Top => self[0].clone().into(),
            Side::Right => {
                let mut swapped = self.clone();
                swapped.transpose();
                swapped[swapped.height() - 1].clone().into()
            }
            Side::Bottom => self[self.height() - 1].clone().into(),
        }
    }
}

/// Contains a list of all possible tiles for a specific position within the generator's grid
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TileSet<const P: usize>(Vec<Tile<P>>);

impl<const P: usize> TileSet<P> {
    /// Creates a new tile set
    pub const fn new(tiles: Vec<Tile<P>>) -> Self {
        Self(tiles)
    }

    /// Returns a reference to the tile set's possible tiles
    pub const fn tiles(&self) -> &Vec<Tile<P>> {
        &self.0
    }
    /// Returns a mutable reference to the tile set's possible tiles
    pub fn tiles_mut(&mut self) -> &mut Vec<Tile<P>> {
        &mut self.0
    }
    /// Returns the total number of possible tiles within the tile set
    pub fn len(&self) -> usize {
        self.tiles().len()
    }
    /// Returns `true` if the total number of possible tiles is exactly one
    pub fn is_collapsed(&self) -> bool {
        self.len() == 1
    }
    /// Returns `true` if the tile set does not contain any possible tiles
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the provided tile is compatible with any of the tile set's possible tiles on the given side
    pub fn connects(&self, tile: &Tile<P>, side: Side) -> bool {
        self.tiles().iter().any(|t| t.connects(tile, side))
    }
    /// Removes the provided tile from the tile set's list of possible tiles
    pub fn remove(&mut self, tile: &Tile<P>) {
        self.tiles_mut().retain(|t| t != tile);
    }
    /// Collapses the tile set into the provided tile
    pub fn collapse(&mut self, tile: &Tile<P>) {
        self.tiles_mut().retain(|t| t == tile);
    }
}

/// Implements the wave function collapse algorithm
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Generator<const P: usize>(Grid<TileSet<P>>);

impl<const P: usize> Generator<P> {
    /// Creates a new generator
    pub fn new(width: usize, height: usize, mut tiles: Vec<Tile<P>>) -> Self {
        tiles.dedup();
        Self(Grid::new(width, height, TileSet::new(tiles)))
    }

    /// Returns a reference to the generator's grid
    pub const fn grid(&self) -> &Grid<TileSet<P>> {
        &self.0
    }
    /// Returns a mutable reference to the generator's grid
    pub fn grid_mut(&mut self) -> &mut Grid<TileSet<P>> {
        &mut self.0
    }
    /// Returns the grid's total number of possible tiles
    pub fn entropy(&self) -> usize {
        self.grid()
            .iter()
            .map(|(_, s)| s.len())
            .reduce(|a, b| a + b)
            .unwrap_or_default()
    }
    /// Returns `true` if the grid has only one possible state
    pub fn is_collapsed(&self) -> bool {
        self.grid().iter().all(|(_, s)| s.is_collapsed())
    }
    /// Returns `true` if the grid is entirely empty; this should never happen
    pub fn is_empty(&self) -> bool {
        self.grid().iter().all(|(_, s)| s.is_empty())
    }
    /// Returns `true` if the grid contains any empty tile sets; this should never happen, but is more likely than `is_empty()` returning `true`
    pub fn is_any_empty(&self) -> bool {
        self.grid().iter().any(|(_, s)| s.is_empty())
    }

    /// Returns a list of possible positions that are directly adjacent to the provided positon
    pub fn adjacent(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
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
    /// Returns a position for the tile set with the lowest number of possible tiles
    pub fn next_position(&self) -> Result<(usize, usize), Error> {
        let mut tiles = self
            .grid()
            .iter()
            .filter(|(_, s)| !s.is_collapsed())
            .collect::<Vec<_>>();

        let entropy = tiles
            .iter()
            .min_by_key(|(_, s)| s.len())
            .ok_or(Error::MissingSet)?
            .1
            .len();

        tiles.retain(|(_, s)| s.len() == entropy);

        if tiles.is_empty() {
            Err(Error::NoValidSet)
        } else {
            let index = thread_rng().gen_range(0..tiles.len());
            let (position, _) = tiles.get(index).ok_or(Error::MissingSet)?;
            Ok(*position)
        }
    }
    /// Collapses the tile set at the provided coordinates to a random possible tile, factoring its weight
    pub fn collapse(&mut self, x: usize, y: usize) -> Result<(), Error> {
        let set = self
            .grid_mut()
            .get_mut(x, y)
            .map_err(|_| Error::MissingSet)?;

        if set.is_empty() {
            Err(Error::EmptySet)
        } else {
            let weights = set.tiles().iter().map(Tile::weight);
            let weights = WeightedIndex::new(weights).map_err(|_| Error::InvalidWeight)?;
            let index = thread_rng().sample(weights);
            let tile = *set.tiles().get(index).ok_or(Error::MissingTile)?;

            set.collapse(&tile);
            Ok(())
        }
    }
    /// Updates all tile sets surrounding the provided position until all affected sets have been updated
    pub fn propogate(&mut self, x: usize, y: usize) -> Result<usize, Error> {
        let mut stack = vec![(x, y)];
        let mut loops = 0_usize;

        while let Some((x1, y1)) = stack.pop() {
            let set = self
                .grid()
                .get(x1, y1)
                .map_err(|_| Error::MissingSet)?
                .clone();

            for (x2, y2) in self.adjacent(x1, y1) {
                let side = Side::relative((x1, y1), (x2, y2));
                let other = self
                    .grid_mut()
                    .get_mut(x2, y2)
                    .map_err(|_| Error::MissingSet)?;

                for tile in other.tiles().clone() {
                    if !set.connects(&tile, side) {
                        other.remove(&tile);

                        if !stack.contains(&(x2, y2)) {
                            stack.push((x2, y2));
                        }
                    }
                }
            }

            loops += 1;
        }

        Ok(loops)
    }
    /// Steps the generator once
    pub fn step(&mut self) -> Result<bool, Error> {
        let (x, y) = self.next_position()?;

        self.collapse(x, y)?;
        self.propogate(x, y)?;

        if self.is_any_empty() {
            Err(Error::EmptySet)
        } else {
            Ok(self.is_collapsed())
        }
    }
    /// Runs the generator
    pub fn run(&mut self, silent: bool) -> Result<Grid<Tile<P>>, Error> {
        if !silent {
            println!(
                "Generating... ({}x{})",
                self.grid().width(),
                self.grid().height()
            );
        }

        let mut cycles = 0_usize;
        let mut props = 0_usize;

        while !self.is_collapsed() {
            if !silent {
                println!("C: {cycles}\tP: {props}\tE: {}", self.entropy());
            }

            let (x, y) = self.next_position()?;

            self.collapse(x, y)?;
            props += self.propogate(x, y)?;
            cycles += 1;

            if self.is_any_empty() {
                return Err(Error::EmptySet);
            }
        }

        if !silent {
            println!("Generation completed; took {cycles} cycles");
        }

        Ok(self.grid().clone().map(|s| s.tiles()[0]))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TileGenerator<const P: usize>(Vec<Tile<P>>);

impl<const P: usize> TileGenerator<P> {
    /// Creates a new tile generator
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Returns a reference to the tile generator's tiles
    pub const fn tiles(&self) -> &Vec<Tile<P>> {
        &self.0
    }
    /// Returns a mutable reference to the tile generator's tiles
    pub fn tiles_mut(&mut self) -> &mut Vec<Tile<P>> {
        &mut self.0
    }
    /// Returns the next valid tile identifier
    pub fn next_id(&self) -> usize {
        match self.tiles().len() {
            0 => 0,
            l => self.tiles()[l - 1].id() + 1,
        }
    }

    /// Adds the provided list of tiles into the tile generator
    pub fn append(&mut self, tiles: &[Tile<P>]) {
        self.tiles_mut().extend_from_slice(tiles);
    }
    /// Adds all tiles that could be parsed from the provided value into the generator
    pub fn generate<T: ToTiles<P>>(&mut self, value: &T, layer: usize, weight: usize) {
        let tiles = value.to_tiles(self.next_id(), layer, weight);
        self.append(&tiles);
        self.tiles_mut().dedup();
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TileData {
    pub id: usize,
    pub version: usize,
    pub tiles: Vec<TileSource>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TileSource {
    pub source: String,
    pub layer: usize,
    pub weight: usize,
    pub nodes: Vec<Vec<usize>>,
}
