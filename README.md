
# Parsing a bitcoin transaction
Using the bitcoin-cli, you can do getrawtransaction <transactionid>
This will return a string of hex which conforms to a subtle and arcane
schema. From this string can be parsed the following information about
the transaction.

1. If you hash the raw text, twice, (using sha256) you can get back to the origianl
   transaction id you passed in.
2. The first 4 bytes are the version number.
3. The next 'n' bytes determine how many inputs the transaction has.
   TOOD explain exactly how to determine the value of 'n'.
4. With the number of inputs, set up an array of inputs, and read off the txid, vout, script_sig and sequence.
   The script_sig requires you to do another 'find how many bytes to handle' dance.
5. Then you do a similar thing for outputs.

note: borrowing some code from
https://github.com/fiatjaf/bitcoin-transaction-hex-decoder with the
intent of learning how to parse a btc transaction.

note: this code so far only works with 'simple' transactions, not segwit
