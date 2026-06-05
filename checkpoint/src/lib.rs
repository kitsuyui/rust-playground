mod id;
use crate::id::Id;

/// A snapshot represents a point-in-time state.
///
/// Implementors must ensure that `id()` returns a stable, unique identifier
/// within the snapshot collection.
pub trait SnapshotRecord<CheckpointID: Id> {
    fn id(&self) -> &CheckpointID;
}

/// A checkpoint represents a named point in a snapshot history graph.
///
/// The intended model is a DAG (directed acyclic graph) similar to a Git
/// commit graph: each checkpoint has zero or more parents and is associated
/// with exactly one snapshot.
///
/// # Invariants
///
/// Implementors are responsible for upholding these invariants. They are not
/// enforced by the type system, but violations cause undefined behavior in
/// any graph traversal over checkpoints.
///
/// - **No self-reference**: `id()` must not appear in `parent_ids()`.
///   Use [`has_self_reference`](CheckpointRecord::has_self_reference) to detect
///   this at runtime.
/// - **No dangling parents**: every ID returned by `parent_ids()` must
///   correspond to an existing `CheckpointRecord` in the same collection.
/// - **Valid snapshot**: `snapshot_id()` must correspond to an existing
///   `SnapshotRecord` in the associated snapshot collection.
pub trait CheckpointRecord<SnapshotID: Id, CheckpointID: Id> {
    fn id(&self) -> &CheckpointID;
    fn parent_ids(&self) -> Vec<&CheckpointID>;
    fn snapshot_id(&self) -> &SnapshotID;

    /// Returns `true` if this checkpoint's own ID appears in `parent_ids()`.
    ///
    /// A well-formed checkpoint must never reference itself as a parent.
    /// Callers performing graph traversal should check this before entering a
    /// traversal loop to guard against trivial cycles.
    fn has_self_reference(&self) -> bool {
        self.parent_ids().contains(&self.id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
    struct U64Id(u64);

    impl Id for U64Id {}

    #[derive(Debug, PartialEq, Eq)]
    struct U64SnapshotRecord {
        id: U64Id,
    }

    impl SnapshotRecord<U64Id> for U64SnapshotRecord {
        fn id(&self) -> &U64Id {
            &self.id
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct U64CheckpointRecord {
        id: U64Id,
        parent_ids: Vec<U64Id>,
        snapshot_id: U64Id,
    }

    impl CheckpointRecord<U64Id, U64Id> for U64CheckpointRecord {
        fn id(&self) -> &U64Id {
            &self.id
        }

        fn parent_ids(&self) -> Vec<&U64Id> {
            self.parent_ids.iter().collect()
        }

        fn snapshot_id(&self) -> &U64Id {
            &self.snapshot_id
        }
    }

    #[test]
    fn test_basis() {
        let snapshot_id = U64Id(1);
        let snapshot = U64SnapshotRecord { id: snapshot_id };
        assert_eq!(snapshot.id(), &snapshot_id);

        let checkpoint_id = U64Id(2);
        let checkpoint = U64CheckpointRecord {
            id: checkpoint_id,
            parent_ids: vec![U64Id(1)],
            snapshot_id: U64Id(1),
        };

        assert_eq!(checkpoint.id(), &checkpoint_id);
        assert_eq!(checkpoint.parent_ids(), vec![&U64Id(1)]);
        assert_eq!(checkpoint.snapshot_id(), &U64Id(1));
    }

    #[test]
    fn test_has_self_reference_false_for_valid_checkpoint() {
        let checkpoint = U64CheckpointRecord {
            id: U64Id(2),
            parent_ids: vec![U64Id(1)],
            snapshot_id: U64Id(1),
        };
        assert!(!checkpoint.has_self_reference());
    }

    #[test]
    fn test_has_self_reference_true_for_self_referencing_checkpoint() {
        let checkpoint = U64CheckpointRecord {
            id: U64Id(2),
            parent_ids: vec![U64Id(2)], // points to itself — invariant violation
            snapshot_id: U64Id(1),
        };
        assert!(checkpoint.has_self_reference());
    }

    #[test]
    fn test_has_self_reference_false_for_root_checkpoint() {
        let checkpoint = U64CheckpointRecord {
            id: U64Id(1),
            parent_ids: vec![],
            snapshot_id: U64Id(1),
        };
        assert!(!checkpoint.has_self_reference());
    }
}
