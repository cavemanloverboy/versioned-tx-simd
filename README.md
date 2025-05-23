---
simd: 0300
title: Message Headers with Compute Budget Metadata
authors:
  - Cavey Cool
category: Standard
type: Interface
status: Idea/Review
created: 2025-05-20
feature: N/A
---

## Summary

We introduce three new (candidate) versions for transaction format aiming at
reducing the transaction footprint for compute budget instructions.

## Motivation

The use of compute budget instructions has become ubiquitous. These instructions
currently have a nontrivial serialized footprint and are presently wasteful in
their implementation. The information in these instructions can be compacted
significantly, enabling users to fit a bit more data in their transaction payload.

## Detailed Design

### v1: Fixed Fields for Compute Unit Limit & Price

- **Change**: `MessageHeader` is extended to include two new fields:
  - `compute_unit_price` (u64)
  - `compute_unit_limit` (u32)
- **Serialization**: These fields are serialized immediately before the existing
  three `u8` signature counters.

### v2: Fixed Fields for Compute Unit Limit & Price, Loaded Data & Heap Requests

- **Change**: `MessageHeader` is extended to include four new fields:
  - `compute_unit_price` (u64)
  - `compute_unit_limit` (u32)
  - `loaded_accounts_data_limit` (u32)
  - `requested_heap_bytes` (u32)
- **Serialization**: These fields are serialized immediately before the existing
  three `u8` signature counters.

### v3: Dynamic Header

- **Change**: Introduce a new `ComputeBudgetHeader` struct at the front of the
    message, containing:
  - `flags: u8` bitmask indicating which compute budget fields are present.
  - Optional fields (`Option<u32>` or `Option<u64>`) for the parameters
- **Serialization**:
  1. Emit `flags` byte.
  2. For each bit set in `flags`, serialize the corresponding field in order
  without additional tags.
  3. Follow with the existing `MessageHeader` (three `u8` counters) and the
    rest of the message.

## Alternatives Considered

I am proposing all options considered.

## Impact

- **DApp Developers**: Clients can still submit legacy/v0, but can opt in to the
  new format to save some bytes.
- **Core Contributors**: Banking/Runtime must support parsing new versions.

## Security Considerations

- validators **MUST** reject messages with unknown budget `flags` bits (v3).

## Drawbacks

- Slight complexity in serializer/deserializer logic, particularly for v3.

## Backwards Compatibility

- `VersionedTransaction` has space in the variant discriminant to support these
  versioned messsages.
