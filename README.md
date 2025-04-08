# QR Code API

> HTTP API for quickly generating QR codes

The intractable Swagger UI documentation is found here: https://qr.tuukka.net

## Summary

Create a QR Code with a simple HTTP GET request. Since it's a GET request, it can be easily embedded in HTML or Markdown files just like this one.

This QR code:

![QR Code to tuukka.net](https://qr.tuukka.net/svg/https://tuukka.net)

is created like so:

`![QR Code to tuukka.net](https://qr.tuukka.net/svg/https://tuukka.net)`

No additional setup is needed. The image format is specified with the first path parameter, so no headers or body parameters are required to make it fully compatible with HTML/Markdown embedding. The content in the second path parameter could be a URL like in this example, a normal string, or some other URI. [Here](https://www.webfx.com/blog/web-design/qr-codes-uri-schemes/) is a blog post about additional URI schemes QR codes. And here is a QR code to it:

![Blog post about QR codes and URI schemes](https://qr.tuukka.net/svg/https://www.webfx.com/blog/web-design/qr-codes-uri-schemes/)

## Technical rambling

The application itself is quite simple. It uses the [qrcode](https://crates.io/crates/qrcode) Rust crate to generate the QR codes. The API is built using the [Axum](https://crates.io/crates/axum) Rust web framework.

[Docker](https://www.docker.com) is used to create the development and production environments. To minimize the size of the production image, the [Alpine](https://en.m.wikipedia.org/wiki/Alpine_Linux) base image of Rust is used to build the binary. [Musl](https://en.m.wikipedia.org/wiki/Musl), the C standard library implementation used in Alpine Linux, allows the creation of [statically linked binaries](https://en.m.wikipedia.org/wiki/Static_build). These statically linked binaries can run without external dependencies from the operating system, so they can also run in the empty [`scratch`](https://hub.docker.com/_/scratch/) docker base image. As musl is used by default on Alpine and cargo can detect the build environment's system architecture, the same image can still be used to create [multi-arch Docker images](https://docs.docker.com/build/building/multi-platform/).
