## Bitcoin transactions
This is a study project. It parses the structure of a raw bitcoin transaction into a Rust struct.
It can also build a raw transaction hex from a Rust struct.

### Notes
1. This code so far only works with pre-segwite transactions.
2. The tests require a bitcoin full node, with the connection parameters set as
environment variables.

