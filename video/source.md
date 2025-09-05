# Source commands

This file contains not important information for me to run other commands of GStreamer.

Avec MP4 output (pas de lag):

```bash
$ GST_DEBUG="fpsdisplaysink:5" gst-launch-1.0 -e -v udpsrc port=11111 ! application/x-rtp,payload=96,media="video",encoding-name="H264",clock-rate="90000" ! rtpjitterbuffer drop-on-latency=true latency=1 ! rtph264depay ! tee name=t t. ! queue ! h264parse ! mp4mux ! filesink location=out.mp4 t. ! queue ! h264parse ! fpsdisplaysink video-sink=fakesink sync=true text-overlay=false
```

For the source:
```bash
ffmpeg -re -f h264 -i train_5M.avi -preset veryfast -vcodec copy -tune zerolatency -f rtp "rtp://127.0.0.1:11111?pkt_size=1100"
```

# Source

ffmpeg -re -i ~/big_buck_bunny_480p_stereo.avi -c:v libx264 -preset veryfast -tune zerolatency -c:a aac -f hls -hls_time 6 -hls_list_size 0 -hls_segment_filename segment_%03d.ts playlist.m3u8