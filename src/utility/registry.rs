use bevy::utils::HashMap;

/// Stores registered values
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Registry<T: PartialEq>(HashMap<String, T>);

impl<T: PartialEq> Registry<T> {
    /// Creates a new registry
    ///
    /// # Examples
    /// ```rust
    /// let mut registry = Registry::new();
    ///
    /// assert!(registry.is_empty());
    /// ```
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Returns the total number of pairs in the registry
    ///
    /// # Examples
    /// ```rust
    /// let mut registry = Registry::new();
    ///
    /// assert_eq!(registry.len(), 0);
    ///
    /// registry.insert("item_1".to_string(), 69);
    /// registry.insert("item_2".to_string(), 420);
    ///
    /// assert_eq!(registry.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// Returns `true` if the registry is empty
    ///
    /// # Examples
    /// ```rust
    /// let mut registry = Registry::new();
    ///
    /// assert!(registry.is_empty());
    ///
    /// registry.insert("item".to_string(), 1337);
    ///
    /// assert!(!registry.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if the registry contains the provided key-value pair
    ///
    /// # Examples
    /// ```rust
    /// let mut registry = Registry::new();
    /// let key = "item".to_string();
    /// let value = 2048;
    ///
    /// assert!(!registry.contains(&key, &value));
    ///
    /// registry.insert(key.clone(), value);
    ///
    /// assert!(registry.contains(&key, &value));
    /// ```
    pub fn contains(&self, key: &String, value: &T) -> bool {
        self.0.iter().any(|p| p == (key, value))
    }
    /// Returns `true` if the registry contains the provided key
    ///
    /// # Examples
    /// ```rust
    /// let mut registry = Registry::new();
    /// let key = "item".to_string();
    ///
    /// assert!(!registry.contains_key(&key));
    ///
    /// registry.insert(key.clone(), 5318008);
    ///
    /// assert!(registry.contains_key(&key));
    /// ```
    pub fn contains_key(&self, key: &String) -> bool {
        self.0.contains_key(key)
    }
    /// Returns `true` if the registry contains the provided value
    ///
    /// # Examples
    /// ```rust
    /// let mut registry = Registry::new();
    /// let value = -0xdead_beef;
    ///
    /// assert!(!registry.contains_value(&value));
    ///
    /// registry.insert("dead beef".to_string(), value);
    ///
    /// assert!(registry.contains(&value));
    /// ```
    pub fn contains_value(&self, value: &T) -> bool {
        self.0.iter().any(|(_, v)| v == value)
    }
    /// Inserts the provided key-value pair into the registry
    ///
    /// # Examples
    /// ```rust
    /// let mut registry = Registry::new();
    /// let key = "item".to_string();
    /// let value = 2048;
    ///
    /// assert!(!registry.contains(&key, &value));
    ///
    /// registry.insert(key.clone(), value);
    ///
    /// assert!(registry.contains(&key, &value));
    /// ```
    pub fn insert(&mut self, key: String, value: T) {
        self.0.insert(key, value);
    }

    /// Returns `true` if both registries contain the same key-value pairs
    ///
    /// # Examples
    /// ```rust
    /// let key = "item".to_string();
    /// let value = 69_420;
    ///
    /// let mut registry_a = Registry::new();
    /// registry_a.insert(key.clone(), value);
    ///
    /// let mut registry_b = Registry::new();
    /// registry_b.insert(key.clone(), value);
    ///
    /// assert!(registry_a.is_synced_with(&registry_b));
    /// ```
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
