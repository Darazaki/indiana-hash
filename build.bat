cargo build --release
mt -manifest .\indiana-hash.manifest -outputresource:./target/release/indiana-hash.exe;1
strip ./target/release/indiana-hash.exe

