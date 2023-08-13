# 🇸4 (Sharded Shared State Storage)

a very simple implementation of an in-memory data storage like redis which is based on sharded shared sate storage design pattern using standard `HashMap` as the shared data

# 🧪 Usage 

```cargo run --bin s4```

# 🛠️ Tools 

* tokio select to receive data from jobq channels asyncly 
* tokio spawn to handle incoming packets asyncly and concurrently
* tokio mutex to acquire the lock on the shared data between tokio green threads asyncly
* tokio jobq channels to move data between tokio green threads asyncly

* redis for streaming over pubsub channels

# 🚧 WIPs

* implement proper sharding and replication algorithms like assigning each data of a shard to a slot owned by a node by sending them through the threads using jobq channels 

* streaming over IO future objects of bytes using redis pubsub and streams, ws actor and tokio concepts (tcp, spawn, select, jobq channels)
