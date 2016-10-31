# TotalRecallDB

WTF is this?

Think of it as a cross between pub/sub and a SQL database.  This is really just a POC to build something more awesome later, likely an alternative to Kafka.

## Building

Building requires nightly Rust due to macros and compiler plugins.

cargo build --release

## Using Embedded Mode

Currently the only thing that can be done, because I haven't written any socket code.  Probably won't for a while either.



## Inserting Data



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

