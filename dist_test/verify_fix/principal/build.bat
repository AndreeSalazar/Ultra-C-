@echo off
setlocal EnableDelayedExpansion
set BASE=principal
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat"
if exist "src\pch.cpp" (
  cl.exe /c /nologo /std:c++17 /EHsc /W4 /WX /permissive- /I include /Yc"pch.hpp" /Fp"build\obj\\pch.pch" src\pch.cpp /Fo"build\obj\\pch.obj"
)
set SRCS=
for %%F in (src\*.cpp) do (
  if /I not "%%~nxF"=="pch.cpp" (
    set SRCS=!SRCS! "%%F"
  )
)
cl.exe /nologo /std:c++17 /EHsc /W4 /WX /permissive- /I include /Yu"pch.hpp" /Fp"build\obj\\pch.pch" !SRCS! build\obj\pch.obj /Fo"build\obj\\" /Fe"build\bin\%BASE%.exe"
