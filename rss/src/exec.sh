#!/bin/bash

QLOGDIR="/rss_feed" RUST_LOG=trace fc-flow-file-transfer --src $FC_SRC_IP:$FC_SRC_PORT --fc-timer $FC_TIMER --cert-path /certs --mc-addr $MC_IP:$MC_PORT $FLEXICAST --ctl-ack-delay 0 --fc-cwnd $FC_CCA --transfer-kind file,socket,/rss_feed/$RSS_SOCKET_NAME --unicast

sudo -E ./amtrelayd -c docker0 -r 130.104.229.58 -a 130.104.229.58 -d

sudo -E ./target/debug/rss $RSS_FEED_URL $RSS_SHARED_DIR/$RSS_SOCKET_NAME