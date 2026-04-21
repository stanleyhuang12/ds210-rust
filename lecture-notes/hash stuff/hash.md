

### Collision
If there are collisions, then there are multiple strategies for mitigating 
- Probing
Each bucket entry: (key, value, hash)
Use a deterministic algorithm to find an open bucket
- Inserting:
entry h(k) busy: try h(k)+1, h(k)+2, etc.
insert into first empty
- Searching:
try h(k), h(k)+1, h(k)+2, etc.
stop when found or empty entry

### hash functions

Hashmap 
- We hash the key or index to use it as a seed value 
    - Similar values should not have similar seed value, instead they should be vastly different 
    - Hash functions are unique but if people know the hash function it can create denial of service attacks 
- We take the seed value and use it as part of a pseudorandom generator 
- This gives us the hash value, which we then modulo with to find where the bucket might be located
- The bucket give us a quick way to identify where the key-value pair may live. 
- The function then will iterate through all the key-value pair in that specific bucket to get a result 

Hashset (no need for buckets)
- We want to keep track of the entire set of keys and don't need 

```rust
let mapp = Hashset::new(); 
```








