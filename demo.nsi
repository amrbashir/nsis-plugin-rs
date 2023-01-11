
Name "demo"
OutFile "demo.exe"
ShowInstDetails show
Unicode true

!addplugindir "./target/release"
!addplugindir "$%CARGO_TARGET_DIR%/release"
!addplugindir "$%CARGO_BUILD_TARGET_DIR%/release"
!addplugindir "./target/debug"
!addplugindir "$%CARGO_TARGET_DIR%/debug"
!addplugindir "$%CARGO_BUILD_TARGET_DIR%/debug"
!addplugindir "./target/i686-pc-windows-msvc\release"
!addplugindir "$%CARGO_TARGET_DIR%/i686-pc-windows-msvc\release"
!addplugindir "$%CARGO_BUILD_TARGET_DIR%/i686-pc-windows-msvc\release"
!addplugindir "./target/i686-pc-windows-msvc\debug"
!addplugindir "$%CARGO_TARGET_DIR%/i686-pc-windows-msvc\debug"
!addplugindir "$%CARGO_BUILD_TARGET_DIR%/i686-pc-windows-msvc\debug"

Section
    nsis_plugin::greet "NSIS"
    Pop $1
    DetailPrint $1
SectionEnd