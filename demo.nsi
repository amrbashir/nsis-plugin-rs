
Name "demo"
OutFile "demo.exe"
ShowInstDetails show
Unicode true

!addplugindir "./target/release"
!addplugindir "./target/debug"
!addplugindir "./target/i686-pc-windows-msvc\release"
!addplugindir "./target/i686-pc-windows-msvc\debug"

Section
    nsis_plugin::greet "NSIS"
    Pop $1
    DetailPrint $1
SectionEnd