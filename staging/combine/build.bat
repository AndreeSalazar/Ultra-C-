@echo off
setlocal EnableDelayedExpansion
set BASE=combine
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat"
if exist "src\pch.cpp" (
  cl.exe /c /nologo /std:c++17 /EHsc /W4 /WX /permissive- /I include /Yc"pch.hpp" /Fp"build\obj\\pch.pch" src\pch.cpp /Fo"build\obj\\pch.obj"
)
cl.exe /nologo /std:c++17 /EHsc /W4 /WX /permissive- /I include /Yu"pch.hpp" /Fp"build\obj\\pch.pch" src\*.cpp build\obj\pch.obj /Fo"build\obj\\" /Fe"build\bin\%BASE%.exe"
