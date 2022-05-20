@echo off
cargo clean
rmdir /q /s docs
cargo doc -q
xcopy /e /y /q target\doc\* .\docs\
xcopy /e /y /q patch\* .\docs\
start /b /wait powershell.exe -command " 'cryp.ren' | Out-File -NoNewLine -Encoding ASCII .\docs\CNAME"