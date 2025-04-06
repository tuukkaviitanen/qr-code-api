# QR Code API

> API for quickly generating a QR code

## Summary

Create a QR Code with a simple HTTP GET request. Since it's a GET request, it can be easily embedded in Markdown or HTML files just like this one.

This QR code:

![QR Code to tuukka.net](https://qr.tuukka.net/qr/https://tuukka.net)

is created like so:

`![QR Code to tuukka.net](https://qr.tuukka.net/qr/https://tuukka.net)`

No additional setup is needed. The image is by default, an SVG, but a PNG can also be created by changing the `Accept` header to `image/png`. The content could be an URL like this one, a normal string, or some other URI. [Here](https://www.webfx.com/blog/web-design/qr-codes-uri-schemes/) is a blog post about additional URI schemes QR codes. And here is a QR code to it:

![Blog post about QR codes and URI schemes](https://qr.tuukka.net/qr/https://www.webfx.com/blog/web-design/qr-codes-uri-schemes/)
