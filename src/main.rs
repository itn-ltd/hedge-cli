mod hedge;

fn main() {
    //hedge::action("open")
    //    .expect("Failed to open hedge.");
    hedge::set_source(&["/mnt/data/one", "/mnt/data/two"], Some("labeltest"))
    .expect("Failed");

    hedge::open().expect("Failed to open hedge");
    hedge::quit().expect("Failed to close hedge.");
}
