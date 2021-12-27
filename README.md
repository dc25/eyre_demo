NAME
====

**eyre** — Demonstrates a couple ways to handle errors.

SYNOPSIS
========

**eyre** \[**-v** _value_to_parse_\] 

DESCRIPTION
===========
Gets a value argument from the command line.  Attempts to parse that value as float.   Prints the result and then reports an error anyway.

NOTES
=====
Currently using stable-eyre to get backtrace working in stable rust.

To enable back trace, export RUST_BACKTRACE=1 in the runtime environment.

EXAMPLES
========
```
eyre -v 7

eyre -v 7b

export RUST_BACKTRACE=1

eyre -v 7b
```

