use std::process::Command;

fn main() {
    // 将 data 打包成tar.gzip
    Command::new("tar")
        .args(["zcvf", "data.tar.gz", "data"])
        .spawn()
        .unwrap();
}
