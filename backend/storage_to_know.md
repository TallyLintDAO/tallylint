## Summary on  canister backup/restore problem
We would like to announce that we started looking into the canister backup/restore problem. We are currently in the design phase and are looking for your input on:

Importance of the problem for you and your use cases. This would help us to prioritise this relative to other problems.
Your thoughts on the potential snapshot-based solution presented here.
Your suggestions of alternative solutions.
Background & Problem statement
Currently, there is no easy way on the Internet Computer to backup data in the event of corruption or data loss, and to restore it by reverting to a previous state. Developers must manually implement a way to serialise the state of the canister, download it off-chain, and then manually upload it if they need to restore the data. This approach is error-prone, not scalable, expensive, and cannot be easily done in a reasonable amount of time.

To address this issue, the IC should provide a way for canisters to take snapshots of their state and restore them when necessary. Additionally, it would be ideal if the data could be exported and imported to and from a local environment, but this would require further engineering effort and can be implemented in future iterations.

By solving this problem at the protocol level, controllers can fix a broken canister by rolling back to a previously saved snapshot in the event of a bug. This is a common problem encountered, especially when upgrading a canister.

## Potential solution
At this stage, our current idea is to concentrate solely on developing a prototype that offers endpoints for capturing on-chain snapshots of canister states and loading the snapshots to canisters. A snapshot will naturally consist of both the stable memory and the heap.

Of course, the initial iteration will have a few constraints, which will help to make the problem as straightforward as possible while providing an improved developer experience.

Only controllers have the authority to take a snapshot and restore it.
While it is not explicitly enforced, creating a snapshot only after stopping the canister is recommended. This follows the same principle as with upgrading a canister, as making sense of the callbacks may not be possible.
During the first iteration, only one snapshot per canister will be allowed, and taking a new snapshot will replace the old one. In later iterations, we may expand this feature to enable multiple snapshots per canister.
Below, you can find an API sketch for interacting with the IC when there is a need to take a snapshot or recover the state from a snapshot identified by snapshot_id.
```
type timestamp = nat;

type bytes = nat;

type snapshot = record {

    id: snapshot_id;
    
    taken_on: timestamp;
    
    label: opt text;
    
    total_size: bytes;
    
    // Checksum for correctness verification.
    
    checksum: blob;

};

service: {

    // Takes a snapshot of the given canister's state.
    
    take_snapshot: (canister_id, label: opt text) -> (snapshot_id);
    
    // Loads the snapshot to the canister identified by `canister_id`.
    
    load_snapshot: (snapshot_id, canister_id) -> ();
    
    // List the snapshots of the canister.
    
    list_snapshots: (canister_id) -> (vec snapshot);
    
    // Deletes the snapshot with the given ID.
    
    delete_snapshot: (snapshot_id) -> ();

}
```
## Future outlook
In the future, it should be possible to incorporate support for more features on top of the existing proposal. Potential additional features that could be implemented include:

Endpoint for downloading a snapshot of a canister to the local environment.
Endpoint for uploading a snapshot from a local environment to the IC.
Ability to create new canisters based on snapshots.
These new features are useful in scenarios such as downloading snapshots to a local environment for debugging or backup purposes, restoring canister states from an off-chain backup or creating new canisters with the data from previously taken snapshots.

However, implementing these features would require considerable engineering effort, including developing tools for manipulating local snapshots, debugging and inspecting data from a snapshot.

## What we are asking the community
Please let us know if the problem of backup/restore is important for you. It would be great if you could share your use case and requirements. We would also like to know if the snapshot-based solution would work for you.

We welcome any alternative proposals that you may have. Thank you for taking the time to share your thoughts with us.