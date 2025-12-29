# Estructura del Proyecto
 
 Este proyecto ha sido generado con una estructura organizada:
 
 - **/src**: Contiene los archivos de código fuente C++ (.cpp).
 - **/include**: Contiene los archivos de cabecera (.hpp).
 - **/build**: Directorio para archivos generados durante la compilación.
   - **/obj**: Archivos objeto (.obj) temporales.
   - **/bin**: Ejecutable final del juego.
 
 Para compilar, ejecute `build.bat` (Windows) o `./build.sh` (Linux/Mac).
 
 ## Modos y Flags útiles
 - `--emit classic|unity|hybrid` selecciona el modo de emisión.
 - `--release` habilita optimizaciones: `/O2` (MSVC) o `-O2` (g++/clang++).
 - `--watch` recompila cuando cambian los `.upp` (modo carpeta).
 - `--bench`/`--staging` generan reporte de métricas en `build/report.json`.
 
 ## Puentes (Bridge) — DLL/SO para integraciones
 Si se generó con `--bridge`, también se construye una librería compartida:
 - Windows: `build/bin/principal.dll`
 - Linux: `build/bin/libprincipal.so`
 
 Ejemplos de uso:
 
 ### Python (ctypes)
 ```python
 import ctypes, os, sys
 libpath = os.path.join('build', 'bin', 'principal.dll' if sys.platform.startswith('win') else 'libprincipal.so')
 lib = ctypes.CDLL(libpath)
 lib.principal_run()
 lib.hola_greet()
 ```
 
 ### Node.js (ffi-napi o N-API)
 ```js
 const os = require('os');
 const path = require('path');
 const ffi = require('ffi-napi');
 const libname = os.platform().startsWith('win') ? 'principal.dll' : 'libprincipal.so';
 const libpath = path.join('build', 'bin', libname);
 const lib = ffi.Library(libpath, {
   'principal_run': [ 'void', [] ],
   'hola_greet': [ 'void', [] ]
 });
 lib.principal_run();
 lib.hola_greet();
 ```
