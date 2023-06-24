Ferric
====


A very primitive key value object store server
that is somewhat compatible with redis clients.
This is **not going to be a production grade software**, 
and I am not reinventing any wheel here, 
this is **just a project for personal learning**.

Language of choice: **Rust**, **because that's what I am learning**.

<u>**Features to implement:**</u>

- String: SET, GET and EXISTS
- Hash: HSET, HGET
- Geospatial using KD/Quad: GEOADD and GEOSEARCH
- Probabilistic: BLOOM_FILTER and/or HYPERLOGLOG
- Lists: LPUSH, LPOP, RPUSH and RPOP
- Set: SADD, SREM, SISMEMBER
- Sorted Set: ZADD, ZRANGE, ZRANK


