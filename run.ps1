$day = ("" + $Args[0]).PadLeft(2, "0")
$part = $Args[1]

cd "day$day"
cargo run --manifest-path="part$part/Cargo.toml"
cd ..
