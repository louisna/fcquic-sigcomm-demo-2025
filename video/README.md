# Big Buck Bunny.... IN THE BROWSER?!

Yes, you read well!
But, ok, let's be honest, there is a small hack here, it's not literally in the browser.
I provide a local proxy to be able to browse the video on the browser, but before that, the data is collected using Flexicast QUIC through a standalone binary.

The idea of this experiment is to get the Big Buck Bunny video stream using Multicast WiFi to show that it can actually work!
You will receive the video content encoded in HLS, so you'll get the manifest as well as the segments.
Then, you can run the [video proxy server](/video-proxy/) server to be able to browse the video locally.

All you have to do is enter the following command, assuming that you cloned this repo in your `~/` directory (or update the path to the `.env_bbb` file):

```bash
$ docker run --env-file ~/fcquic-sigcomm-demo-2025/video/.env_bbb --net host -v ./shared:/shared -t louisna/deployment-fcquic
```

Pay attention that we use a different environment file because this is a different application on top of Flexicast QUIC.
This will store in `./shared` the manifest and segment files. Then, the [proxy](/video-proxy/) can be used to start a local server and then browse the video on your favorite browser on [http://localhost:5554/playlist.m3u8](http://localhost:5554/playlist.m3u8):

```bash
$ cargo run --release --manifest-path ~/fcquic-sigcomm-demo-2025/video-proxy/Cargo.toml -- ~/fcquic-sigcomm-demo-2025/video/shared/
```

Again, here we assume that the repository is in your `~/`.

# Computing live streaming metrics

Another application (that I call `ssim`) sends another stream using RTP on top of Flexicast QUIC. This uses the `.env` file and the `compose.yaml` you see here.
Theoretically, just starting with `docker compose up` should enable you to receive the stream and process it with GStreamer (through `gst-launch`).
However, the docker image of GStreamer that I built seems not to be compatible with a lot of architectures, and I cannot find a working standard GStreamer image...

This use case will not show the video as a live stream (like the Big Buck Bunny one), but will compute a `.avi` file of the video and additionally log stall events reported from GStreamer.
This will be useful because we will be able to measure:
- The Structured Similarity (SSIM): the quality of the rendered video compared to the original video.
- Stall events, reported by the logs of GStreamer.

## You still want to participate to this use case?

So, if you want to participate to this use-case, the best you can do is install GStreamer by yourself on your laptop... I'm sorry for this.
Use the following command:

```bash
apt-get install -y libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libgstreamer-plugins-bad1.0-dev gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav gstreamer1.0-tools gstreamer1.0-x gstreamer1.0-alsa gstreamer1.0-gl gstreamer1.0-gtk3 gstreamer1.0-qt5 gstreamer1.0-pulseaudio
```

Then, you can start the Docker of Flexicast QUIC alone:

```bash
$ docker run --env-file ~/fcquic-sigcomm-demo-2025/video/.env --net host -v ./shared:/shared -t louisna/deployment-fcquic
```

And manually GStreamer in another terminal:

```bash
$ GST_DEBUG="fpsdisplaysink:5" gst-launch-1.0 -e -v udpsrc port=22222 ! application/x-rtp,payload=96,media="video",encoding-name="H264",clock-rate="90000" ! rtpjitterbuffer drop-on-latency=true latency=100 ! rtph264depay ! tee name=t t. ! queue ! h264parse ! avimux ! filesink location=~/fcquic-sigcomm-demo-2025/video/shared/out.avi t. ! queue ! h264parse ! fpsdisplaysink video-sink=fakesink sync=true text-overlay=false &> ~/fcquic-sigcomm-demo-2025/video/shared/gst_log.log 2>&1
```

You will see two files in the `shared/` repository:
- `out.avi`: the video computed by GStreamer, with a default playback buffer of 100 ms (you can conroll it in the previous command by changing the `latency=100`).
- `gst_log.log`: a log file computed by GStreamer. This file contains interesting informations to compute the fps rate of the video, to compute stall events.

Then, you can upload these two files to our website, and we will use them to compute metrics that will enable us to make beautiful graphs.
We cannot compute the SSIM directly, otherwise you would need the original video (this is not huge) and run the SSIM computation script yourself (this is huge).
To upload the two files, you can `curl` them to our site, assuming this time that you are in the `shared/` repository:

```bash
curl -i -X POST -H "Content-Type: media" -F "data=@gst_log.log" http://localhost:8000/gstlog
curl -i -X POST -H "Content-Type: media" -F "data=@out.avi" http://localhost:8000/video
```

If you reach the end of this README, you are awesome, and I really want to thank you for your time! Please, do not forget to fill in the form to remain updated!