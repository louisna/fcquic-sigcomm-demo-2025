# Big Buck Bunny.... IN THE BROWSER?!

Yes, you read well!
But, ok, let's be honest, there is a small hack here, it's not literally in the browser.
I provide a local proxy to be able to browse the video on the browser, but before that, the data is collected using Flexicast QUIC through a standalone binary.

The idea of this experiment is to get the Big Buck Bunny video stream using Multicast WiFi to show that it can actually work!
You will receive the video content encoded in HLS, so you'll get the manifest as well as the segments.
Then, you can run the [video proxy server](/video-proxy/) server to be able to browse the video locally.

All you have to do is enter the following command:

```bash
$ docker run --env-file ~/fcquic-sigcomm-demo-2025/recv/.env_bbb --net host -v ./shared:/shared -t louisna/deployment-fcquic
```

Pay attention that we use a different environment file because this is a different application on top of Flexicast QUIC.
This will store in `./shared` the manifest and segment files. Then, the [proxy](/video-proxy/) can be used to start a local server and then browse the video on your favorite browser on [http://localhost:5554/playlist.m3u8](http://localhost:5554/playlist.m3u8):

```bash
$ cargo run --release --manifest-path ~/fcquic-sigcomm-demo-2025/video-proxy/Cargo.toml
```

# Computing live streaming metrics

```bash
$ GST_DEBUG="fpsdisplaysink:5" gst-launch-1.0 -e -v udpsrc port=11111 ! application/x-rtp,payload=96,media="video",encoding-name="H264",clock-rate="90000" ! rtpjitterbuffer drop-on-latency=true latency=1 ! rtph264depay ! tee name=t t. ! queue ! h264parse ! avimux ! filesink location=out.avi t. ! queue ! h264parse ! fpsdisplaysink video-sink=fakesink sync=true text-overlay=false
```

Avec MP4 output (pas de lag):

```bash
$ GST_DEBUG="fpsdisplaysink:5" gst-launch-1.0 -e -v udpsrc port=11111 ! application/x-rtp,payload=96,media="video",encoding-name="H264",clock-rate="90000" ! rtpjitterbuffer drop-on-latency=true latency=1 ! rtph264depay ! tee name=t t. ! queue ! h264parse ! mp4mux ! filesink location=out.mp4 t. ! queue ! h264parse ! fpsdisplaysink video-sink=fakesink sync=true text-overlay=false
```

For the source:
```bash
ffmpeg -re -f h264 -i train_5M.avi -preset veryfast -vcodec copy -tune zerolatency -f rtp "rtp://127.0.0.1:11111?pkt_size=1100"
```

# Computing the SSIM and the rebuffering time with a video stream