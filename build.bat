cargo build --release
mt -manifest .\libui-rs\ui-sys\libui\windows\_rc2bin\libui.manifest -outputresource:./target/release/indiana-hash.exe;1
strip ./target/release/indiana-hash.exe

