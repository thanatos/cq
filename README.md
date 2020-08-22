# `cq`

`cq` is a tool for working with [CBOR] data. 

Current, `cq` can:

* Emit a human-readable textual representation of the input data.
* Convert CBOR to JSON.

In the future, I'd like this tool to support a query language (like it's
namesake — `jq`), however, it does not do that today. However, if you have both
this tool and `jq` installed, such functionality can be approximated with,

```
$ cq <input -o json | jq
```

## What's CBOR?

[CBOR] is an RFC-standardized format for exchanging data, similar to JSON, but
in binary, with more built-in types. (CBOR has support for dates, binary blobs,
IEEE floating points distinct from integers, and more.)

[CBOR]: https://cbor.io/
