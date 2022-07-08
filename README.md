## Bitcoin transaction parser
This is a learning project, with the Base58 course.
It requires a bitcoin full node, with the connection parameters set as
environment variables.

### Parsing a bitcoin transaction
This code parses the hex returned by
```
  bitcoin-cli getrawtransaction <transactionid>
```
...which conforms to a subtle and arcane schema.

### Notes
1. If you hash the raw text, twice, (using sha256) you can get back to the origianl
   transaction id you passed in.
2. The first 4 bytes are the version number.
3. The next 'n' bytes determine how many inputs the transaction has.
   TOOD explain exactly how to determine the value of 'n' - this is the
"compact size", which is also used to determine the number of outputs,
and the ScriptSig
4. With the number of inputs, set up an array of inputs, and read off the txid, vout, script_sig and sequence.
5. Then you do a similar thing for outputs.
6. This code so far only works with 'simple' transactions, not segwit
7. Borrowed code from https://github.com/fiatjaf/bitcoin-transaction-hex-decoder for byte manipulation and endian conversion

