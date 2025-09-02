use axum::{Router, response::IntoResponse, routing::get};
use std::path::PathBuf;
use std::{
    fs,
    net::UdpSocket,
    sync::{Arc, Mutex},
    thread,
};
use tokio::{fs::File, io::AsyncReadExt};

const UDP_PORT: u16 = 6789;
const HTTP_PORT: u16 = 5554;
const SEGMENT_DIR: &str = "/tmp/segments";
const PLAYLIST_PATH: &str = "/tmp/segments/playlist.m3u8";

fn start_udp_listener(shared_state: Arc<Mutex<SegmentWriter>>) {
    thread::spawn(move || {
        let socket = UdpSocket::bind(("127.0.0.1", UDP_PORT)).expect("Failed to bind UDP socket");
        let mut buf = [0u8; 65535];

        loop {
            if let Ok((size, _src)) = socket.recv_from(&mut buf) {
                let data = &buf[..size];
                let mut writer = shared_state.lock().unwrap();
                writer.write_segment(data).unwrap();
            }
        }
    });
}

struct SegmentWriter {
    counter: usize,
}

impl SegmentWriter {
    fn new() -> Self {
        fs::create_dir_all(SEGMENT_DIR).unwrap();
        Self { counter: 0 }
    }

    fn write_segment(&mut self, data: &[u8]) -> std::io::Result<()> {
        let filename = format!("segment_{:05}.ts", self.counter);
        let filepath = format!("{}/{}", SEGMENT_DIR, filename);
        fs::write(&filepath, data)?;
        self.counter += 1;
        self.update_playlist()?;
        Ok(())
    }

    fn update_playlist(&self) -> std::io::Result<()> {
        let mut playlist = String::from(
            "#EXTM3U\n#EXT-X-VERSION:3\n#EXT-X-TARGETDURATION:10\n#EXT-X-MEDIA-SEQUENCE:0\n",
        );
        for i in 0..self.counter {
            playlist.push_str("#EXTINF:10.0,\n");
            playlist.push_str(&format!("segment_{:05}.ts\n", i));
        }
        fs::write(PLAYLIST_PATH, playlist)
    }
}

async fn serve_file(path: PathBuf) -> impl IntoResponse {
    match tokio::fs::read(path).await {
        Ok(content) => ([("Content-Type", "video/MP2T")], content).into_response(),
        Err(_) => ([("Content-Type", "text/plain")], "Not Found").into_response(),
    }
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(SegmentWriter::new()));
    start_udp_listener(state);

    let app = Router::new()
        .route(
            "/playlist.m3u8",
            get(|| async {
                match File::open(PLAYLIST_PATH).await {
                    Ok(mut file) => {
                        let mut contents = Vec::new();
                        file.read_to_end(&mut contents).await.unwrap();
                        (
                            [("Content-Type", "application/vnd.apple.mpegurl")],
                            contents,
                        )
                            .into_response()
                    }
                    Err(_) => {
                        ([("Content-Type", "text/plain")], "Playlist not found").into_response()
                    }
                }
            }),
        )
        .route(
            "/{filename}",
            get(
                |axum::extract::Path(filename): axum::extract::Path<String>| async move {
                    let mut path = PathBuf::from(SEGMENT_DIR);
                    path.push(&filename);
                    serve_file(path).await
                },
            ),
        );

    println!("Server running at http://localhost:{}", HTTP_PORT);
    let listener = tokio::net::TcpListener::bind(
        format!("127.0.0.1:{}", HTTP_PORT)
            .parse::<std::net::SocketAddr>()
            .unwrap(),
    )
    .await
    .unwrap();
    axum::serve::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
