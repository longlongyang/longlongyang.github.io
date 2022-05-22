@echo off
cargo clean
IF exist docs ( rmdir /q /s docs )
cargo doc -p blog -q
xcopy /e /y /q target\doc\* .\docs\
xcopy /e /y /q patch\* .\docs\
cargo run -p rearrange_tool -q
start /b /wait powershell.exe -command " 'cryp.ren' | Out-File -NoNewLine -Encoding ASCII .\docs\CNAME"
