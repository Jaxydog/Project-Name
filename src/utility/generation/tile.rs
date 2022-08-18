use std::array::IntoIter;

use serde::{Deserialize, Serialize};

/// Represents one of four possible sides of a tile
#[repr(usize)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    #[default]
    Top = 0,
    Left = 1,
    Right = 2,
    Bottom = 3,
}

impl Side {
    /// Returns the side opposite to this one
    pub const fn opposite(self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Bottom => Self::Top,
        }
    }

    /// Returns the side closest to the `end` relative to the `start` position
    pub const fn relative_x(start: usize, end: usize) -> Self {
        if start > end {
            Self::Right
        } else {
            Self::Left
        }
    }
    /// Returns the side closest to the `end` relative to the `start` position
    pub const fn relative_y(start: usize, end: usize) -> Self {
        if start > end {
            Self::Bottom
        } else {
            Self::Top
        }
    }
    /// Returns the side closest to the `end` relative to the `start` position
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

impl IntoIterator for Side {
    type Item = Self;
    type IntoIter = IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        let mut base = [Self::Top, Self::Left, Self::Right, Self::Bottom];
        base.rotate_left(self.into());
        base.into_iter()
    }
}

/// Represents one of four possible rotations of a tile
#[repr(usize)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rotation {
    #[default]
    D0 = 0,
    D90 = 1,
    D180 = 2,
    D270 = 3,
}

impl Rotation {
    /// Returns the rotation opposite to this one
    pub const fn opposite(self) -> Self {
        match self {
            Self::D0 => Self::D180,
            Self::D90 => Self::D270,
            Self::D180 => Self::D0,
            Self::D270 => Self::D90,
        }
    }
}

impl From<Rotation> for usize {
    fn from(rotation: Rotation) -> Self {
        rotation as Self
    }
}

impl IntoIterator for Rotation {
    type Item = Self;
    type IntoIter = IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        let mut base = [Self::D0, Self::D90, Self::D180, Self::D270];
        base.rotate_left(self.into());
        base.into_iter()
    }
}

/// Represents one side of a tile, for ensuring neighboring tiles are compatible
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Socket<const P: usize>([usize; P]);

impl<const P: usize> Socket<P> {
    /// Creates a new socket
    pub const fn new(nodes: [usize; P]) -> Self {
        Self(nodes)
    }

    /// Returns the socket's nodes
    pub const fn nodes(&self) -> [usize; P] {
        self.0
    }

    /// Returns a copy of the socket with its nodes reversed
    pub fn reversed(&self) -> Self {
        let mut nodes = self.nodes();
        nodes.reverse();
        nodes.into()
    }
    /// Returns `true` if the socket is symmetrical
    pub fn is_symmetric(&self) -> bool {
        self == &self.reversed()
    }
}

impl<const P: usize> Default for Socket<P> {
    fn default() -> Self {
        Self::new([usize::default(); P])
    }
}

impl<const P: usize, T: Into<usize>> From<[T; P]> for Socket<P> {
    fn from(nodes: [T; P]) -> Self {
        Self::new(nodes.map(Into::into))
    }
}

/// Contains transformation information for tile generation
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Transform(Rotation, bool, bool);

impl Transform {
    /// Creates a new transform
    pub const fn new(rotation: Rotation, flip_x: bool, flip_y: bool) -> Self {
        Self(rotation, flip_x, flip_y)
    }

    /// Returns the transform's rotation
    pub const fn rotation(&self) -> Rotation {
        self.0
    }
    /// Returns whether the transform is flipped across the x axis
    pub const fn flip_x(&self) -> bool {
        self.1
    }
    /// Returns whether the transform is flipped across the y axis
    pub const fn flip_y(&self) -> bool {
        self.2
    }
}

/// Contains tile information for use in generation
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tile<const P: usize>((usize, usize), [Socket<P>; 4], usize);

impl<const P: usize> Tile<P> {
    /// Creates a new tile
    pub const fn new(id: usize, layer: usize, sockets: [Socket<P>; 4], weight: usize) -> Self {
        Self((id, layer), sockets, weight)
    }

    /// Returns the tile's identifier
    pub const fn id(&self) -> usize {
        self.0 .0
    }
    /// Returns the tile's layer identifier
    pub const fn layer(&self) -> usize {
        self.0 .1
    }
    /// Returns the tile's sockets
    pub const fn sockets(&self) -> [Socket<P>; 4] {
        self.1
    }
    /// Returns the tile's socket on the given side
    pub const fn socket_on(&self, side: Side) -> Socket<P> {
        self.1[side as usize]
    }
    /// Returns the tile's randomness weight
    pub const fn weight(&self) -> usize {
        self.2
    }

    /// Returns `true` if the tile can connect to the other tile on the given side
    pub fn connects_to(&self, other: &Self, side: Side) -> bool {
        self.layer() == other.layer() && self.socket_on(side) == other.socket_on(side.opposite())
    }
}

/// Contains information for both a tile and a tile transformation
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransformedTile<const P: usize>(Tile<P>, Transform);

impl<const P: usize> TransformedTile<P> {
    /// Creates a new transformed tile
    pub const fn new(tile: Tile<P>, transform: Transform) -> Self {
        Self(tile, transform)
    }

    /// Returns the transformed tile's tile information
    pub const fn tile(&self) -> Tile<P> {
        self.0
    }
    /// Returns the transformed tile's transformation information
    pub const fn transform(&self) -> Transform {
        self.1
    }

    /// Returns the transformed tile's tile with rotations applied
    pub fn rotated(&self) -> Tile<P> {
        let mut rotated = self.tile();
        rotated.1.rotate_right(self.transform().rotation().into());
        rotated
    }
    /// Returns the transformed tile's tile with flips applied
    pub fn flipped(&self) -> Tile<P> {
        let mut flipped = self.tile();
        let left = usize::from(Side::Left);
        let right = usize::from(Side::Right);
        let top = usize::from(Side::Top);
        let bottom = usize::from(Side::Bottom);

        if self.transform().flip_x() {
            flipped.1.swap(left, right);
            flipped.1[top] = flipped.1[top].reversed();
            flipped.1[bottom] = flipped.1[bottom].reversed();
        }
        if self.transform().flip_y() {
            flipped.1.swap(top, bottom);
            flipped.1[left] = flipped.1[left].reversed();
            flipped.1[right] = flipped.1[right].reversed();
        }

        flipped
    }
    /// Returns the transformed tile's tile with all transformations applied
    pub fn transformed(&self) -> Tile<P> {
        Self::new(self.flipped(), self.transform()).rotated()
    }
}

/// List of tiles and all of their possible transformations
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TileSet<const P: usize>(usize, Vec<TransformedTile<P>>);

impl<const P: usize> TileSet<P> {
    /// Creates a new tile set
    pub const fn new(id: usize) -> Self {
        Self(id, Vec::new())
    }
    /// Creates a new tile set that contains the provided tiles
    pub fn new_with(id: usize, tiles: &[TransformedTile<P>]) -> Self {
        Self(id, tiles.to_vec())
    }

    /// Returns the tile set's identifier
    pub const fn id(&self) -> usize {
        self.0
    }
    /// Returns a reference to the tile set's tiles
    pub const fn tiles(&self) -> &Vec<TransformedTile<P>> {
        &self.1
    }
    /// Returns a mutable reference to the tile set's tiles
    pub fn tiles_mut(&mut self) -> &mut Vec<TransformedTile<P>> {
        &mut self.1
    }
    /// Returns the total number of tiles within the set
    pub fn len(&self) -> usize {
        self.tiles().len()
    }
    /// Returns `true` if the total number of tiles in the set is `0`
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the next available identifier for a tile within the set
    pub fn next_id(&self) -> usize {
        self.tiles()
            .iter()
            .max_by_key(|t| t.tile().id())
            .map_or(0, |t| t.tile().id() + 1)
    }
    /// Generates a socket from the provided vector
    fn gen_socket(raw: Vec<usize>) -> [usize; P] {
        let mut nodes = [0; P];

        raw.into_iter()
            .enumerate()
            .take(P)
            .for_each(|(i, n)| nodes[i] = n);

        nodes
    }

    /// Adds the provided transformed tile into the set
    pub fn add_transformed(&mut self, tile: TransformedTile<P>) {
        self.tiles_mut().push(tile);
    }
    /// Adds the provided list of transformed tiles into the set
    pub fn add_all_transformed(&mut self, tiles: &[TransformedTile<P>]) {
        self.tiles_mut().extend_from_slice(tiles);
    }
    /// Adds the provided tile into the set, generating possible transformations
    pub fn add_tile(&mut self, tile: Tile<P>) {
        let mut tiles = Vec::new();

        for rotation in Rotation::D0 {
            tiles.push(TransformedTile::new(
                tile,
                Transform::new(rotation, false, false),
            ));
            tiles.push(TransformedTile::new(
                tile,
                Transform::new(rotation, true, false),
            ));
            tiles.push(TransformedTile::new(
                tile,
                Transform::new(rotation, false, true),
            ));
            tiles.push(TransformedTile::new(
                tile,
                Transform::new(rotation, true, true),
            ));
        }

        self.add_all_transformed(&tiles);
    }
    /// Adds the provided list of tiles into the set, generating possible transformations for each
    pub fn add_all_tiles(&mut self, tiles: &[Tile<P>]) {
        tiles.iter().for_each(|t| self.add_tile(*t));
    }
    /// Adds the provided raw tile into the set, generating a tile and possible transformations
    pub fn add_raw(&mut self, raw: Raw) {
        let id = self.next_id();
        let mut sockets = [Socket::default(); 4];

        sockets[usize::from(Side::Top)] = Self::gen_socket(raw.nodes.0).into();
        sockets[usize::from(Side::Left)] = Self::gen_socket(raw.nodes.1).into();
        sockets[usize::from(Side::Right)] = Self::gen_socket(raw.nodes.2).into();
        sockets[usize::from(Side::Bottom)] = Self::gen_socket(raw.nodes.3).into();

        self.add_tile(Tile::new(id, raw.layer, sockets, raw.weight));
    }
    /// Adds the provided list of raw tiles into the set, generating a tile and possible transformations for each
    pub fn add_all_raws(&mut self, raws: &[Raw]) {
        raws.iter().for_each(|r| self.add_raw(r.clone()));
    }
}

/// Defines a file header for raw tile information
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawFile {
    pub id: usize,
    pub version: usize,
    pub precision: usize,
    pub tiles: Vec<Raw>,
}

/// Defines raw tile information
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Raw {
    pub source: String,
    pub layer: usize,
    pub weight: usize,
    pub nodes: (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>),
}
