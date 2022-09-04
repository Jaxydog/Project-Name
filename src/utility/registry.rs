use std::fmt::Display;

use bevy::utils::HashMap;

/// Value used to identify a value stored in a registry
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier(String, String);

impl Identifier {
    /// Creates a new identifier
    pub const fn new(namespace: String, path: String) -> Self {
        Self(namespace, path)
    }

    /// Returns a reference to the identifier's namespace
    pub const fn namespace(&self) -> &String {
        &self.0
    }
    /// Returns a reference to the identifier's path
    pub const fn path(&self) -> &String {
        &self.1
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace(), self.path())
    }
}

/// Stores registered values
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Registry<T: PartialEq>(HashMap<Identifier, T>);

impl<T: PartialEq> Registry<T> {
    /// Creates a new registry
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Returns the total number of pairs in the registry
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// Returns `true` if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if the registry contains the provided key-value pair
    pub fn contains(&self, key: &Identifier, value: &T) -> bool {
        self.0.iter().any(|p| p == (key, value))
    }
    /// Returns `true` if the registry contains the provided key
    pub fn contains_key(&self, key: &Identifier) -> bool {
        self.0.contains_key(key)
    }
    /// Returns `true` if the registry contains the provided value
    pub fn contains_value(&self, value: &T) -> bool {
        self.0.iter().any(|(_, v)| v == value)
    }
    /// Inserts the provided key-value pair into the registry
    pub fn insert(&mut self, key: Identifier, value: T) {
        self.0.insert(key, value);
    }

    /// Returns `true` if both registries contain the same key-value pairs
    pub fn is_synced_with(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for (key, value) in self.0.iter() {
            if !other.contains(key, value) {
                return false;
            }
        }

        true
    }
}
