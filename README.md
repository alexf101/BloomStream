## Backlog

- [x] parse flags for array size and pass-through
- [x] create bitvector array to store cache using trait/type
- [] accept rw traffic via server
- [] create second array with passthrough 10% (flag)
- [] for v1, accept memory allocation directly
- [] benchmark/test - hit it with random elements
- [] export hit/miss rate and fullness and epoch


- [] for v2, accept how long to last

## Algorithm

For streaming bloom filter implementation, we want to know:
- How full we allow the bloom filter to get before replacement
- Either:
  - How much memory we want to allocate
  - How long we want it to last
