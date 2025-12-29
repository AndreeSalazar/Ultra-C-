@echo off
setlocal EnableDelayedExpansion
set BASE=combine
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat"
cl.exe /nologo /std:c++17 /EHsc /W4 /WX /permissive- /I include src\*.cpp /Fo"build\obj\\" /Fe"build\bin\%BASE%.exe"
