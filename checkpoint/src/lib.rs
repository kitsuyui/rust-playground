mod id;
use crate::id::Id;

pub trait SnapshotRecord<CheckpointID: Id> {
    fn id(&self) -> &CheckpointID;
}

pub trait CheckpointRecord<SnapshotID: Id, CheckpointID: Id> {
    fn id(&self) -> &CheckpointID;
    fn parent_ids(&self) -> Vec<&CheckpointID>;
    fn snapshot_id(&self) -> &SnapshotID;
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
}
