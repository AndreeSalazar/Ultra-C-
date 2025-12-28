# Ultra C++

Transpilador en Rust que convierte un lenguaje OOP puro con sintaxis tipo Python (.upp) a C++ moderno (C++17), generando headers (.hpp), sources (.cpp) y un ejecutable de prueba opcional.

## Características
- Sintaxis OOP tipo Python con indentación:
  - `class Nombre:`
  - Campos a nivel de clase: `campo: Tipo`
  - Métodos: `def metodo(self, p: Tipo) -> Ret:`
  - `return "..." + self.campo`
- Generación C++:
  - Clase, constructor con todos los `fields`
  - Métodos que retornan `String` mapeados a `std::string`
- Carpeta de salida por archivo:
  - `dist/<base>/<base>.hpp`, `<base>.cpp`, `main.cpp`, `build.bat`
- Compilación integrada:
  - Detecta Visual Studio Build Tools (VsDevCmd.bat) y compila con `cl.exe`
  - Alternativas: intenta `g++` o `clang++` si están en PATH

## Requisitos
- Rust (cargo)
- Windows:
  - Visual Studio Build Tools (C++), o
  - MinGW-w64 (g++), o
  - LLVM (clang++)

## Uso Rápido
1. Compilar el transpilador:
   - `cargo build --release`
2. Crear un archivo `.upp`, por ejemplo `hola.upp`:
   ```
   class Hola:
       nombre: String
       def saludo(self) -> String:
           return "Hola " + self.nombre
   ```
3. Generar C++:
   - `./target/release/ultracpp.exe hola.upp`
   - Salida:
     - `dist/hola/hola.hpp`
     - `dist/hola/hola.cpp`
     - `dist/hola/main.cpp`
     - `dist/hola/build.bat`
4. Compilar C++ (MSVC):
   - Abrir "Developer Command Prompt for VS" o usar PowerShell
   - `cd dist/hola`
   - `.\build.bat`
   - Ejecutar:
   - `.\hola.exe`

## CLI
- `ultracpp <input.upp> [outdir]`
  - Si no se especifica `outdir`, se usa `dist/`
  - Se crea una carpeta `dist/<base>/` por cada `.upp`

## Ejemplo
- Entrada: `hola.upp`
  - [hola.upp](file:///c:/Users/andre/OneDrive/Documentos/Ultra%20C++/hola.upp)
- Generado:
  - Header: [dist/hola/hola.hpp](file:///c:/Users/andre/OneDrive/Documentos/Ultra%20C++/dist/hola/hola.hpp)
  - Source: [dist/hola/hola.cpp](file:///c:/Users/andre/OneDrive/Documentos/Ultra%20C++/dist/hola/hola.cpp)

## Desarrollo
- Código principal:
  - Parser: [src/parser.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra%20C++/src/parser.rs)
  - Generador C++: [src/codegen.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra%20C++/src/codegen.rs)
  - CLI: [src/main.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra%20C++/src/main.rs)
  - Detector MSVC: [src/tool_detector.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra%20C++/src/tool_detector.rs)
- Pruebas:
  - [tests/transpile.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra%20C++/tests/transpile.rs)

## Subir a GitHub
1. Inicializar git:
   - `git init`
2. Revisar estado:
   - `git status`
3. Añadir archivos:
   - `git add .`
4. Crear commit (opcional aquí, o hazlo luego):
   - `git commit -m "Ultra C++: transpilador base"`
5. Crear repositorio en GitHub e incluir remoto:
   - `git remote add origin https://github.com/<tu-usuario>/<tu-repo>.git`
6. Subir:
   - `git push -u origin main`

