mod util;
use std::time;

fn main() {
    let now = time::Instant::now();

    util::find_by_filesystem();

    println!("\n\n\n========= Took {} milliseconds =========", now.elapsed().as_millis());
}
