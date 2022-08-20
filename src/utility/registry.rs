use bevy::utils::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Registry<T: PartialEq>(HashMap<String, T>);

impl<T: PartialEq> Registry<T> {
    /// Creates a new registry
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Returns the total length of the registry
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// Returns `true` if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if the registry contains the provided key-value pair
    pub fn contains(&self, key: &String, value: &T) -> bool {
        self.0.iter().any(|p| p == (key, value))
    }
    /// Returns `true` if the registry contains the provided key
    pub fn contains_key(&self, key: &String) -> bool {
        self.0.contains_key(key)
    }
    /// Returns `true` if the registry contains the provided value
    pub fn contains_value(&self, value: &T) -> bool {
        self.0.iter().any(|(_, v)| v == value)
    }
    /// Inserts the provided key-value pair into the registry
    pub fn insert(&mut self, key: String, value: T) {
        self.0.insert(key, value);
    }

    /// Returns `true` if both registries contain the same key-value pairs
    pub fn is_synced_with(&self, other: &Self) -> bool {
        if self.len() == other.len() {
            for (key, value) in self.0.iter() {
                if !other.contains(key, value) {
                    return false;
                }
            }

            true
        } else {
            false
        }
    }
}
