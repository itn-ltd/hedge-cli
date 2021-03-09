mod hedge;

fn main() {
    hedge::action("open")
        .expect("Failed to open hedge.");
}
