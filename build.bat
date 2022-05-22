@echo off
cargo clean
IF exist docs ( rmdir /q /s docs )
cargo fmt
cargo doc -p blog -q
xcopy /e /y /q target\doc\* .\docs\ >nul
xcopy /e /y /q patch\* .\docs\ >nul
cargo run -p rearrange_tool -q
cargo run -p semantics_tool -q
start /b /wait powershell.exe -command " 'cryp.ren' | Out-File -NoNewLine -Encoding ASCII .\docs\CNAME"
