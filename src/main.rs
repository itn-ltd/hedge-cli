mod hedge;

fn main() {
    hedge::action("hedge://open/")
        .expect("Failed to open hedge.");
}
