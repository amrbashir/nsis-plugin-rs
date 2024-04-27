
Name "demo"
OutFile "demo.exe"
ShowInstDetails show
Unicode true

!addplugindir "./target/i686-pc-windows-msvc\release"
!addplugindir "$%CARGO_TARGET_DIR%/i686-pc-windows-msvc\release"
!addplugindir "$%CARGO_BUILD_TARGET_DIR%/i686-pc-windows-msvc\release"

Section
    nsis_plugin::greet "NSIS"
    Pop $1
    DetailPrint $1
SectionEnd