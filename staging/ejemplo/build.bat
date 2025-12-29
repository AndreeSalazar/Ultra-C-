@echo off
setlocal
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat"
cl.exe /nologo /std:c++17 /EHsc /I include src\*.cpp /Fo"build\obj\\" /Fe"build\bin\ejemplo.exe"
