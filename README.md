# tkrpc

- Super basic RPC client and server demo
- Far from the libraries used in production (gRPC)

## What this teaches

- network programming over TCP (gRPC uses http2)
- doesn't take into scope the usage of code gen (protoc)
- lots of further optimizations to be made (packing / compression, better networking, concurrency)
- further good reads https://beej.us/guide/bgnet/

## Why?

- everything in modern software engineering is abstracted away (and for a good reason)
- being able to deep dive into how these libraries work = growth of knowledge
- what's the case for "rolling my own RPC framework" --> this directly answers that complexity question
