# Flexicast QUIC Deployment at ACM SIGCOMM 2025 demo

This repository contains all information related to the demo of Flexicast QUIC during the ACM SIGCOMM 2025 Demo session at Coimbra, Portugal.

During this demo, people can experiment with Flexicast QUIC in two scenarios:
- File transfer: pushed-based RSS updates of the Linux Kernel Lore mailing list through the Internet using Automatic Multicast Tunneling (AMT, RFC 7450)
- Video stream: pushed-based HLS of the Big Buck Bunny video through Multicast WiFi (Local Area Network)

## Receiving-side: how to connect to the source

For people who whish to connect to the Flexicast QUIC source for either use case, [please follow instructions in the rss/ directory](rss/README.md).

We provide Docker images and environment variables to automate deployment and connection with our source.

__DISCLAMER__: On Windows/macOS, Docker containers are run within a virtual machine. It is thus impossible to connect to the local network of the host through the container, which is required to receive multicast packets... As such, I think the only way to still participate to the demo if you have a Window/macOS computer is to compile from source the binaries. I assure you, this is not big, because it's Rust. And you should have a valid Rust installation. I mean, Rust is nice, so you should work with Rust. So it's not a big deal, is it?

## Hosting your own Flexicast QUIC source

We provide [an open-source extension of Flexicast QUIC based on Cloudflare quiche](https://github.com/IPNetworkingLab/flexicast-quic), which is based on our paper [Taking the Best of Multicast and Unicast with Flexicast QUIC](https://louisna.github.io/publication/2025-ccr-flexicast).
However, this implementation is not up to date.
We will release soon the new implementation to let you play with Flexicast QUIC and host your own implementation!

## Collaboration

We intend to pursue larger-scale experiments of Flexicast QUIC on the Internet during the following months.
If you are interested to collaborate, [please fill out the following form](https://docs.google.com/forms/d/e/1FAIpQLSdGYZBBuPZqZJoADlOamy8Jcxnn3GAst81xBDzIX7IZYfypJA/viewform?usp=header).
I will use your email to contact you in the future, but will not disclore any personnal information to anyone.

## You want to know more about Flexicast QUIC?

Do not hesitate to look at the [Flexicast QUIC](https://louisna.github.io/publication/2025-ccr-flexicast)!

## Cite this work

```bibtex
@article{navarre2025towards,
  title={Towards an Internet Deployment of Flexible Multicast QUIC},
  author={Navarre, Louis and Bonaventure, Olivier},
  journal={" SIGCOMM'25: Proceedings of the SIGCOMM'25 Poster and Demo Sessions"},
  year={2025}
}
```