# DataNode
To start the DataNode simply run
```bash
cargo run identifier name-server-url data-node-port
```

for example:
```
cargo run client-1 http://localhost:3000 8081
```

Each DataNode must use a different identifier and port.

To distribute data over the network run the following command. Replace the port with the port you used for a DataNode.
```bash
curl --request POST '127.0.0.1:8081/distribute' \
--header 'Content-Type: application/json' \
--data-raw '{
	"data": "I am a data-string!!!"
}'
```

To lookup a hash simply run the following command. Replace `hash_to_look_up` the hash to want to seach for.
```bash
curl --location --request GET '127.0.0.1:8083/lookup?hash=hash_to_look_up'
```
