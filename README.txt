# TotalRecallDB

WTF is this?

Think of it as a cross between pub/sub and a SQL database.  This is really just a POC to build something more awesome later, likely an alternative to Kafka.

## Building

Building requires nightly Rust due to macros and compiler plugins.

cargo build --release

Look in target/release for the binary totalrecalldb

## Using Embedded Mode

Currently the only thing that can be done, because I haven't written any socket code.  Probably won't for a while either.

./totalrecalldb test

Will start up a REPL with an embedded server.  Currently only creating streams and inserting data works.



## Doing Stuff


```
[?] embedded> declare stream ts (sensor int, data text );
Stream Created.

[?] embedded> insert into ts set sensor=1, data='test';
Inserted 0
[?] embedded> insert into ts set sensor=1, data='test2';
```


## Storage Format

file per stream

stream header: 128bytes - too much?  what do i even need here?

Row header
    u16 size (2 bytes)
    start id
    end id
    some timestamp?

serialized row data
    field type
    row size (optional?)
    row data

same format is used for the return of results (results are themselves streams) meaning the network format matches the on disk format.  possibly stupid.

