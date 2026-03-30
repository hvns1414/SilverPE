@echo off
setlocal enabledelayedexpansion
cls

:: Banner'ı daha güvenli bir yöntemle yazdırıyoruz
echo  ____  _ _                ____  _____ 
echo / ___^|(_) ^|_   _____ _ __^|  _ \^| ____^|
echo \___ \^| ^| \ \ / / _ \ '__^| ^|_) ^|  _^|  
echo  ___) ^| ^| ^|\ V /  __/ ^|  ^|  __/^| ^|___ 
echo ^|____/^|_^|_^| \_/ \___/^|_^|  ^|_^|   ^|_____^|
echo.
echo [ SilverPE - Build Script ]
echo ---------------------------------

:: mnt klasörünü kontrol et ve oluştur
if not exist "mnt" (
    echo [*] Creating 'mnt' directory...
    mkdir mnt
)

echo [*] Building x64 (SilverPE)...
cargo build --release --target x86_64-pc-windows-msvc
if %ERRORLEVEL% EQU 0 (
    copy /y "target\x86_64-pc-windows-msvc\release\SilverPE.exe" "mnt\SilverPE_x64.exe"
    echo [+] x64 Build Successful.
) else (
    echo [-] x64 Build Failed!
)

echo.
echo [*] Building x86 (SilverPE)...
cargo build --release --target i686-pc-windows-msvc
if %ERRORLEVEL% EQU 0 (
    copy /y "target\i686-pc-windows-msvc\release\SilverPE.exe" "mnt\SilverPE_x86.exe"
    echo [+] x86 Build Successful.
) else (
    echo [-] x86 Build Failed!
)

echo.
echo [+] Done! Check the 'mnt' folder.
pause
