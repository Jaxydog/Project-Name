use std::fmt::{Debug, Display};

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
}

/// Represents one side of a tile
#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
}

impl From<Side> for usize {
    fn from(side: Side) -> Self {
        side as Self
    }
}

/// Represents a fixed number of 90 degree rotations
#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rotation {
    None = 0,
    Once = 1,
    Twice = 2,
    Thrice = 3,
}

impl Rotation {
    /// Returns the rotation 180 degrees away from this one
    pub const fn opposite(self) -> Self {
        match self {
            Self::None => Self::Twice,
            Self::Once => Self::Thrice,
            Self::Twice => Self::None,
            Self::Thrice => Self::Once,
        }
    }
}

impl From<Rotation> for usize {
    fn from(rotation: Rotation) -> Self {
        rotation as Self
    }
}

/// Represents one of three compatibility markers within a socket
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Node(usize, usize);

impl Node {
    /// Creates a new node
    pub const fn new(id: usize, layer: usize) -> Self {
        Self(id, layer)
    }

    /// Returns the node's identifier
    pub const fn id(&self) -> usize {
        self.0
    }
    /// Returns the node's layer identifier
    pub const fn layer(&self) -> usize {
        self.1
    }
}

/// Stores data for one side of a tile, for checking compatibility between two tiles
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Socket(Node, Node, Node);

impl Socket {
    /// Creates a new socket
    pub const fn new(left: Node, center: Node, right: Node) -> Self {
        Self(left, center, right)
    }

    /// Returns the socket's left-most node
    pub const fn left(&self) -> Node {
        self.0
    }
    /// Returns the socket's center node
    pub const fn center(&self) -> Node {
        self.1
    }
    /// Returns the socket's right-most node
    pub const fn right(&self) -> Node {
        self.2
    }
    /// Returns a copy of the socket that has been flipped
    pub const fn flipped(&self) -> Self {
        Self::new(self.right(), self.center(), self.left())
    }

    /// Returns `true` if the socket is symmetrical
    pub fn symmetrical(&self) -> bool {
        self == &self.flipped()
    }
}

/// Represents one possible state within the generator's grid
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tile {
    id: usize,
    sockets: [Socket; 4],
    rotation: Rotation,
    flip_x: bool,
    flip_y: bool,
    weight: usize,
}

impl Tile {
    /// Creates a new tile
    pub const fn new(
        id: usize,
        sockets: [Socket; 4],
        rotation: Rotation,
        flip_x: bool,
        flip_y: bool,
        weight: usize,
    ) -> Self {
        Self {
            id,
            sockets,
            rotation,
            flip_x,
            flip_y,
            weight,
        }
    }

    /// Returns the tile's identifier
    pub const fn id(&self) -> usize {
        self.id
    }
    /// Returns the tile's sockets
    pub const fn sockets(&self) -> [Socket; 4] {
        self.sockets
    }
    /// Returns the tile's rotation
    pub const fn rotation(&self) -> Rotation {
        self.rotation
    }
    /// Returns `true` if the tile is flipped horizontally
    pub const fn x_flipped(&self) -> bool {
        self.flip_x
    }
    /// Returns `true` if the tile is flipped vertically
    pub const fn y_flipped(&self) -> bool {
        self.flip_y
    }
    /// Returns the tile's weight
    pub const fn weight(&self) -> usize {
        self.weight
    }

    /// Returns the socket on the given side
    pub fn socket(&self, side: Side) -> Socket {
        self.sockets()[usize::from(side)]
    }
    /// Returns a copy of the socket that has been rotated if necessary
    pub fn rotated(&self) -> Self {
        let mut rotated = *self;
        rotated.sockets.rotate_right(self.rotation().into());
        rotated.rotation = Rotation::None;
        rotated
    }
    /// Returns a copy of the socket that has been flipped if necessary
    pub fn flipped(&self) -> Self {
        let mut flipped = *self;

        if flipped.x_flipped() {
            flipped.sockets.swap(Side::Left.into(), Side::Right.into());
            flipped.sockets[usize::from(Side::Top)] = flipped.socket(Side::Top).flipped();
            flipped.sockets[usize::from(Side::Bottom)] = flipped.socket(Side::Bottom).flipped();
            flipped.flip_x = false;
        }
        if flipped.y_flipped() {
            flipped.sockets.swap(Side::Top.into(), Side::Bottom.into());
            flipped.sockets[usize::from(Side::Left)] = flipped.socket(Side::Left).flipped();
            flipped.sockets[usize::from(Side::Right)] = flipped.socket(Side::Right).flipped();
            flipped.flip_y = false;
        }

        flipped
    }
    /// Returns a copy of the socket that has been rotated and flipped if necessary
    pub fn transformed(&self) -> Self {
        self.rotated().flipped()
    }
    /// Returns `true` if the provided tile is compatible on the given side
    pub fn connects(&self, tile: Self, side: Side) -> bool {
        let facing = side.opposite();
        let this = self.transformed().socket(side);
        let tile = tile.transformed().socket(facing);

        this == tile
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}

/// Contains a list of all possible states for a specific position in the generator's grid
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TileSet(Vec<Tile>);

impl TileSet {
    /// Creates a new tile set
    pub const fn new(tiles: Vec<Tile>) -> Self {
        Self(tiles)
    }

    /// Returns a reference to the tile set's possible tiles
    pub const fn tiles(&self) -> &Vec<Tile> {
        &self.0
    }
    /// Returns a mutable reference to the tile set's possible tiles
    pub fn tiles_mut(&mut self) -> &mut Vec<Tile> {
        &mut self.0
    }
    /// Returns the total number of possible tiles within the tile set
    pub fn len(&self) -> usize {
        self.tiles().len()
    }
    /// Returns `true` if the tile set only has one possible tile
    pub fn is_collapsed(&self) -> bool {
        self.len() == 1
    }
    /// Returns `true` if the tile set does not have any possible tiles
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the provided tile is compatible with any possible tile on the given side
    pub fn connects(&self, tile: Tile, side: Side) -> bool {
        self.tiles().iter().any(|t| t.connects(tile, side))
    }
    /// Removes the provided tile from the tile set's list of possible tiles
    pub fn remove(&mut self, tile: &Tile) {
        self.tiles_mut().retain(|t| t != tile);
    }
    /// Collapses the tile set into a single possible tile
    pub fn collapse(&mut self, tile: &Tile) {
        self.tiles_mut().retain(|t| t == tile);
    }
}

/// Implements the wave function collapse algorithm
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Generator(Grid<TileSet>);

impl Generator {
    /// Creates a new generator
    pub fn new(mut tiles: Vec<Tile>, width: usize, height: usize) -> Self {
        tiles.dedup();
        Self(Grid::new(width, height, TileSet::new(tiles)))
    }

    /// Returns a reference to the generator's grid
    pub const fn grid(&self) -> &Grid<TileSet> {
        &self.0
    }
    /// Returns a mutable reference to the generator's grid
    pub fn grid_mut(&mut self) -> &mut Grid<TileSet> {
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
                    if !set.connects(tile, side) {
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
    pub fn run(&mut self) -> Result<Grid<Tile>, Error> {
        println!(
            "Generating... ({}x{})",
            self.grid().width(),
            self.grid().height()
        );

        let mut cycles = 0_usize;
        let mut props = 0_usize;

        while !self.is_collapsed() {
            println!("C: {cycles}\tP: {props}\tE: {}", self.entropy());

            let (x, y) = self.next_position()?;

            self.collapse(x, y)?;
            props += self.propogate(x, y)?;
            cycles += 1;

            if self.is_any_empty() {
                return Err(Error::EmptySet);
            }
        }

        println!("Generation completed; took {cycles} cycles");

        Ok(self.grid().clone().map(|s| s.tiles()[0]))
    }
}

/// Handles parsing a value into a node
pub trait AsNode {
    /// Returns the value as a node with the provided layer
    fn as_node(&self, layer: usize) -> Node;
}

impl<T: Clone + Into<usize>> AsNode for T {
    fn as_node(&self, layer: usize) -> Node {
        Node::new(self.clone().into(), layer)
    }
}

/// Handles parsing a value into a node
pub trait AsSocket {
    /// Returns the socket's left-most node
    fn left_node(&self, layer: usize) -> Node;
    /// Returns the socket's center node
    fn center_node(&self, layer: usize) -> Node;
    /// Returns the socket's right-most node
    fn right_node(&self, layer: usize) -> Node;

    /// Returns the value as a socket
    fn as_socket(&self, layers: (usize, usize, usize)) -> Socket {
        Socket::new(
            self.left_node(layers.0),
            self.center_node(layers.1),
            self.right_node(layers.2),
        )
    }
}

impl<T: AsNode> AsSocket for [T] {
    fn left_node(&self, layer: usize) -> Node {
        self[0].as_node(layer)
    }
    fn center_node(&self, layer: usize) -> Node {
        self[self.len() / 2].as_node(layer)
    }
    fn right_node(&self, layer: usize) -> Node {
        self[self.len() - 1].as_node(layer)
    }
}

impl<T: AsNode> AsSocket for Vec<T> {
    fn left_node(&self, layer: usize) -> Node {
        self[0].as_node(layer)
    }
    fn center_node(&self, layer: usize) -> Node {
        self[self.len() / 2].as_node(layer)
    }
    fn right_node(&self, layer: usize) -> Node {
        self[self.len() - 1].as_node(layer)
    }
}

pub trait AsTiles {
    /// Returns the tile's socket on the given side
    fn side_socket(&self, side: Side, layers: (usize, usize, usize)) -> Socket;

    /// Returns the tile's sockets for each side
    fn all_sockets(&self, layers: [(usize, usize, usize); 4]) -> [Socket; 4] {
        [
            self.side_socket(Side::Left, layers[usize::from(Side::Left)]),
            self.side_socket(Side::Top, layers[usize::from(Side::Top)]),
            self.side_socket(Side::Right, layers[usize::from(Side::Right)]),
            self.side_socket(Side::Bottom, layers[usize::from(Side::Bottom)]),
        ]
    }
    /// Returns the value as a tile with no transformations
    fn as_base_tile(&self, id: usize, layers: [(usize, usize, usize); 4], weight: usize) -> Tile {
        let sockets = self.all_sockets(layers);
        Tile::new(id, sockets, Rotation::None, false, false, weight)
    }
    /// Returns all possible tiles that can be parsed from the value
    fn as_tiles(&self, id: usize, layers: [(usize, usize, usize); 4], weight: usize) -> Vec<Tile> {
        let mut tiles = Vec::new();

        for rotation in 0..4_usize {
            let rotation = match rotation {
                1 => Rotation::Once,
                2 => Rotation::Twice,
                3 => Rotation::Thrice,
                _ => Rotation::None,
            };

            for flip_x in 0..=1 {
                let flip_x = flip_x == 1;

                for flip_y in 0..=1 {
                    let flip_y = flip_y == 1;
                    let sockets = self.all_sockets(layers);
                    let tile = Tile::new(id, sockets, rotation, flip_x, flip_y, weight);

                    tiles.push(tile);
                }
            }
        }

        tiles.dedup();
        tiles
    }
}

impl<T: AsNode + Clone> AsTiles for Grid<T> {
    fn side_socket(&self, side: Side, layers: (usize, usize, usize)) -> Socket {
        match side {
            Side::Left => {
                let mut swapped = self.clone();
                swapped.transpose();
                swapped[0].as_socket(layers)
            }
            Side::Top => self[0].as_socket(layers),
            Side::Right => {
                let mut swapped = self.clone();
                swapped.transpose();
                swapped[swapped.height() - 1].as_socket(layers)
            }
            Side::Bottom => self[self.height() - 1].as_socket(layers),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TileGenerator(Vec<Tile>);

impl TileGenerator {
    /// Creates a new tile generator
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Returns a reference to the tile generator's tiles
    pub const fn tiles(&self) -> &Vec<Tile> {
        &self.0
    }
    /// Returns a mutable reference to the tile generator's tiles
    pub fn tiles_mut(&mut self) -> &mut Vec<Tile> {
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
    pub fn append(&mut self, tiles: &[Tile]) {
        self.tiles_mut().extend_from_slice(tiles);
    }
    /// Adds all tiles that could be parsed from the provided value into the generator
    pub fn generate<T: AsTiles>(
        &mut self,
        value: &T,
        layers: [(usize, usize, usize); 4],
        weight: usize,
    ) {
        let id = self.next_id();
        let tiles = value.as_tiles(id, layers, weight);
        self.append(&tiles);
        self.tiles_mut().dedup();
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TileSource {
    pub source: String,
    pub weight: usize,
    pub nodes: Vec<Vec<usize>>,
    pub layers: Vec<(usize, usize, usize)>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TileData {
    pub id: usize,
    pub version: usize,
    pub tiles: Vec<TileSource>,
}
