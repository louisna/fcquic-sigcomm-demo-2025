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

# Big Buck Bunny.... IN THE BROWSER?!

Yes, you read well!
But, ok, let's be honest, there is a small hack here, it's not literally in the browser.
I provide a local proxy to be able to browse the video on the browser, but before that, the data is collected using Flexicast QUIC through a standalone binary.

The idea of this experiment is to get the Big Buck Bunny video stream using Multicast WiFi to show that it can actually work!
You will receive the video content encoded in HLS, so you'll get the manifest as well as the segments.
Then, you can run the [video proxy server](/video-proxy/) server to be able to browse the video locally.

All you have to do is enter the following command:

```bash
docker run --env-file ~/fcquic-sigcomm-demo-2025/recv/.env_bbb --net host -v ./shared:/shared -t louisna/deployment-fcquic
```

Pay attention that we use a different environment file because this is a different application on top of Flexicast QUIC.
This will store in `./shared` the manifest and segment files. Then, the [proxy](/video-proxy/) can be used to start a local server and then browse the video on your favorite browser on [http://localhost:5554/playlist.m3u8](http://localhost:5554/playlist.m3u8):

```bash
cargo run --release --manifest-path ~/fcquic-sigcomm-demo-2025/video-proxy/Cargo.toml
```

