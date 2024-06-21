# X1 XenBlocks Ledger

This program represents a ledger to store XenBlock data and votes on X1 chain

## Interface

- `initialize` Reserved for admin to initialize Global State
- `submit_block` Reserved for the watch tower to send newly found blocks
- `vote_for_block` Public interface method for all miners who shall vote for found blocks

## Data Structures

### XenBlocksState

Attributes related to the ledger as a whole

- `block_id` counter

### XenBlockInfo

Single XenBlock is referenced by a combination of:

- "xen-block" byte-string
- `block id`

Attributes of a single XenBlock

- `key` unique 32-byte sequence resolving to a valid XenBlock hash
- `block_type` regular (XM), XUNI or superblock (X.BLK)
- `miner` base-58-encoded pubkey of a miner who has submitted the newly found block
- `votes` counter of other miners' votes for the block

### VoteInfo

Single Vote is referenced by a combination of:

- "xen-block-vote" byte-string
- `block id`
- `voter` base-58-encoded pubkey of a voter

Attribute of a single vote for a XenBlock

- `key` unique 32-byte sequence resolving to a valid XenBlock hash
- `slot` unique id of a current X1 slot (block)

## Events

### NewBlock

Attributes:

- `id`
- `miner`
- `key`
- `block_type`
- `slot` (TBD if needed)

### NewVote

Attributes:

- `id`
- `voter`
- `key`
- `slot` (TBD if needed)

## Errors

- `AlreadyInitialized`
- `AlreadySubmitted`
- `AlreadyVoted`
- `CannotSelfVote`




