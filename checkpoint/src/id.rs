use std::hash::Hash;

// Id is a type that represents a unique identifier for a snapshot or checkpoint.
// It must implement Eq, Hash, and Ord.
pub trait Id: Sized + PartialEq + Eq + Hash + Copy + Clone + PartialOrd + Ord {}

#[cfg(test)]
mod tests {
    use super::*;

    // test u64 as Id
    #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
    struct U64Id(u64);

    impl Id for U64Id {}

    #[test]
    fn test_equality() {
        let id1 = U64Id(1);
        let id2 = U64Id(2);
        let id3 = U64Id(3);

        // test equality
        assert_eq!(id1, id1);
        assert_eq!(id2, id2);
        assert_eq!(id3, id3);
        assert_ne!(id1, id2);
        assert_ne!(id1, id3);
        assert_ne!(id2, id3);
    }

    #[test]
    fn test_hashable() {
        let id1 = U64Id(1);
        let id2 = U64Id(2);
        let id3 = U64Id(3);

        // Id can be used as a key in a HashMap
        let mut set = std::collections::HashSet::new();
        set.insert(id1);
        assert!(set.contains(&id1));
        assert!(!set.contains(&id2));
        assert!(!set.contains(&id3));

        set.insert(id2);
        assert!(set.contains(&id1));
        assert!(set.contains(&id2));
        assert!(!set.contains(&id3));
    }
}
