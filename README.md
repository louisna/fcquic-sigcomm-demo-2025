# Pull-based RSS feed of the Linux Lore mailing list using Flexicast QUIC

This document describes the procedure for getting updates on the Linux Lore mailing list using Flexicast QUIC (FCQUIC) and Automatic Multicast Tunneling (AMT).

Please refer to the abstract for more details on Flexicast QUIC and Automatic Multicast Tunneling.

We provide a docker-compose file that will automatically start AMT and FCQUIC and connect to our source to get updates through multicast.

The only thing to do is start the containers with the following command from this directory:
```
# docker compose up
```

Please pay attention that the used source code of AMT requires privileged mode to create inner tuntap links.

## Getting the RSS feed.

The feed comes from https://lore.kernel.org/all/new.atom. We use this source as we regularly get new emails.

Starting the containers will initiate a QUIC connection with our source server, and upgrate it to Flexicast if the `FLEXICAST` environment variable is set to `--flexicast`. The containers start with the `.env` file to configure all these variables automatically.

Upon reception of RSS feed, the FCQUIC receiver (you) creates the `data.txt` located in `$(pwd)/shared` directory. This `data.txt` file contains the up to date RSS feed, which can be consulted using classic RSS readers such as `newsboat`.

For example, the following configuration file will allow `newsboat` to read the RSS content received through Flexicast QUIC:

```
file:///home/louisna/fc-file-transfer/deployment/shared/data.txt
```