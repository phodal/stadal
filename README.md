# Stadal

## Architecture (design)

Node.js

 - [neon](https://github.com/neon-bindings/neon) Rust bindings for writing safe and fast native Node.js modules.

RPC

 - [tarpc](https://github.com/google/tarpc) is an RPC framework for rust with a focus on ease of use. Defining a service can be done in just a few lines of code, and most of the boilerplate of writing a server is taken care of for you.
 - [gRPC-rs](https://github.com/tikv/grpc-rs) is a Rust wrapper of gRPC Core. gRPC is a high performance, open source universal RPC framework that puts mobile and HTTP/2 first.

Refs:

 - [Xi Editor](https://github.com/xi-editor/xi-editor) project is an attempt to build a high quality text editor, using modern software engineering techniques. 

## Documents

Library:

 - [heim](https://github.com/heim-rs/heim) is an ongoing attempt to create the best tool for system information fetching (ex., CPU, memory, disks or processes stats) in the Rust crates ecosystem.

[Status library comparison](https://github.com/heim-rs/heim/blob/master/COMPARISON.md)

## Notes

requests

```
{"method":"client_started","params":{}}
```

LICENSE
===

RPC based on [xi-editor](https://github.com/xi-editor/xi-editor) with Apache 2.0 & Inspired by [xi-term](https://github.com/xi-frontend/xi-term)