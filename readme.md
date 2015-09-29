Serve
=====

Serve is a simple command line utility to serve the files in the current
directory via http.

Note: Serve will not show the list of files in the current directory. It
will only serve files (i.e. `localhost` won't show anything, but
`localhost/index.html` will)

### Usage

```
> serve
Listening on port 8080
```

By default serve listens on port 8080, but you can override this by providing
a port number explicitly:

```
> serve 3000
Listening on port 3000
```

### Building it with Cargo

As always, Cargo does its job pretty well:

```
cargo build --release
```

**Caveat**: Serve depends on Iron, which in turn depends on OpenSSL. Therefore,
the build will fail in case Cargo can't find OpenSSL in your system. If you are
unsure about what you have to do, please follow the instructions given in the
[rust-openssl repo](https://github.com/sfackler/rust-openssl).
