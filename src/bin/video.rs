use video::concat;

fn main() {
    concat("/Users/boris/Downloads/test", "mp4")
        .expect("Failed to concat files");
}
