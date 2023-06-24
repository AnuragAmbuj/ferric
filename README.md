Ferric
====


A very primitive key value object store server
that is somewhat compatible with redis clients.
This is **not going to be a production grade software**, 
and I am not reinventing any wheel here, 
this is **just a project for personal learning**.

Language of choice: **Rust**, **because that's what I am learning**.

<u>**Features to implement:**</u>

- P0 String: SET, GET and EXISTS
- P1 Geospatial using KD/Quad: GEOADD and GEOSEARCH
- P3 Probabilistic: BLOOMFILTER and/or HYPERLOGLOG
- P4 Lists: LPUSH, LPOP, RPUSH and RPOP
- P4 Set: SADD, SREM, SISMEMBER
- P4 Hash: HSET, HGET
- P4 Sorted Set: ZADD, ZRANGE, ZRANK


