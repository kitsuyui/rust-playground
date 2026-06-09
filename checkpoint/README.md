# Checkpoint and Snapshot

Traits for modelling a snapshot-and-checkpoint history graph, similar to a
Git commit graph.

## Concepts

- **SnapshotRecord** – a point-in-time state, identified by a unique ID.
- **CheckpointRecord** – a named position in the history DAG. Each checkpoint
  references one snapshot and zero or more parent checkpoints.

## Invariants

The traits do not enforce these at the type level. Implementors must uphold
them to keep graph traversal correct:

| Invariant | Description |
|---|---|
| No self-reference | `id()` must not appear in `parent_ids()` |
| No dangling parents | every ID in `parent_ids()` must exist in the checkpoint collection |
| Valid snapshot | `snapshot_id()` must correspond to an existing `SnapshotRecord` |

`CheckpointRecord::has_self_reference()` provides a runtime check for the
first invariant and should be called before any graph traversal.
