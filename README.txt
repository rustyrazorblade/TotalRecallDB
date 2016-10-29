This database doesn't even have a name, for now, it's stream your face off.

WTF is this?

Think of it as a cross between pub/sub and a SQL database.  This is really just a POC to build something more awesome later, likely an alternative to Kafka.


## Storage Format

file per stream

stream header: 128bytes - too much?  what do i even need here?

Row header
    u16 size (2 bytes)

serialized row data
    field type
    row size (optional?)
    row data

same format is used for the return of results (results are themselves streams) meaning the network format matches the on disk format.  possibly stupid.  

