cargo clean
rmdir /q /s docs
cargo doc
xcopy /e /y target\doc\* .\docs\