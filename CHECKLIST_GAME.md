# Checklist de Verificación: Ultra C++ (Proyecto Base)

## Estado Actual (verificado)
- [x] Entrada explícita: `entry Principal` con `run()` ([principal.upp](file:///c:/Users/andre/OneDrive/Documentos/Ultra%20C++/combine/principal.upp))
- [x] Método auxiliar: `hola.upp()` funcionando con `call hola.upp`
- [x] Sintaxis simple: `print` nativo y `self` implícito
- [x] Namespaces correctos: `Juego`, `Utils`
- [x] Emisión `classic` (headers + main) y `unity` (all.cpp) disponibles
- [x] Auto-includes de dependencias en `.cpp` (clases referenciadas y llamadas estáticas)
- [x] UTF-8 en Windows: `SetConsoleOutputCP(65001)` en `main`
- [x] Validación de `call hola.upp` en el chequeo de tipos
- [x] Compilación MSVC/g++/clang++ mediante `build.bat`/`build.sh`

## Requisitos Funcionales del “Juego” (alineados a Principal)
- [x] Bucle de ejecución base: `Principal.run()`
- [x] Bucle de juego real: `start()` y `loop()` con estado `running`
- [x] Demo rápida: `Hola("como están").greet()`
- [x] Entidades básicas: `Entity`, `Player` con coordenadas y movimiento
- [x] Estado del juego: `running`, `score`, `level`
- [x] Entrada/Salida consola: `std::iostream` vía `print`

## Emisión C++ y Estructura
- [x] Classic: `include/*.hpp`, `src/*.cpp`, `src/entry.cpp` ([main.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra%20C++/src/main.rs))
- [x] Unity: `dist/.../src/all.cpp` opcional
- [x] Hybrid: headers públicos + unity interno
- [x] Scripts de build generados automáticamente (`build.bat`, `build.sh`)

## Robustez y Calidad
- [x] Auto-includes (referencias y llamadas estáticas tipo `Utils::Version::current`)
- [x] Validación de tipos (incluye `FileCall`, tipos no resueltos)
- [x] Manejo de errores en `main` (try/catch)
- [x] Deduplicación de `#include` en headers
- [x] Resolver advertencias “unreachable pattern” en `codegen.rs`
- [x] Flag unificada `--emit classic|unity|hybrid`

## Pruebas
- [x] Classic: `.\ultra.bat combine` (genera headers y main, compila y ejecuta)
- [x] Unity: `cargo run -- combine --unity --compile`
- [x] Inputs variados: unicode adicional, clases nuevas, `import` con versiones

## Uso Correcto de `hola.upp`
- Definir el método dentro de la clase objetivo:
  - En `principal.upp`, `hola.upp()` imprime la versión, instancia `Hola` y muestra `greet()`
- Invocar desde `run()` con `call hola.upp`
- Si no existe, el compilador avisa: define `hola_upp()` o `hola()`

## Próximos Pasos (plan)
- [x] Implementar `--emit classic|unity|hybrid`
- [x] Añadir modo `hybrid`
- [x] Deduplicar includes en headers generados
- [x] Mejorar mensajes de `type_check` para `FileCall`
- [x] Añadir pruebas de unicode y más inputs
- [x] Documentar “Call” (README actualizado)
- [x] Añadir ejemplo de `Entity/Player` y `loop`

## Guía de Uso Rápido
- [x] Classic — `.\ultra.bat combine` (Windows) / `ultracpp combine dist --compile` (Linux/macOS)
- [x] Unity — `cargo run -- combine --unity --compile`
- [x] Hybrid — `cargo run -- combine --emit hybrid --compile`
- [x] Release — activar `/O2` (MSVC) o `-O2` (g++/clang++) automáticamente en `build.*`
- [x] CI — script de integración continua (GitHub Actions) con matrix MSVC/g++/clang++

## Motor del Juego
- [x] Separar `update()` y `draw()` para un ciclo claro
- [x] Delta time y FPS objetivo configurable (ej. 60 FPS)
- [x] Estados `paused`/`resumed` con transiciones seguras
- [x] Input teclado (native `_kbhit`/`_getch`) con mapeo básico
- [x] Física simple y colisiones AABB (rectángulos axis‑aligned)
- [x] Sistema de eventos (suscripción/emisión) para acoplamiento bajo

## Assets y Configuración
- [x] Render ASCII estable en `draw()`
- [x] Sonidos simulados por logs con categorías/prioridad
- [x] Config externo (TOML) para niveles/ajustes con defaults seguros
- [x] Sistema de recursos (carga/descarga) con conteo de referencias
- [x] Validación de config (schema ligero) y errores descriptivos
- [x] Hot-reload de config en modo carpeta `--watch`
- [x] Mapeo de sprites ASCII por categoría (configurable)
- [x] Canal de audio lógico (bus) integrado con `EventBus`
- [x] Perfiles de config (dev/test/prod) con overrides
- [x] Persistencia de high score en archivo simple
- [x] Localización de textos (tabla de recursos de strings)

## Ejemplos y Demos
- [x] Hola mundo — ejecución mínima con `entry Principal`
- [x] Call file — `call hola.upp` con clases auxiliares
- [x] Loop básico — estado `running` y ciclo controlado
- [x] Unicode — salida UTF‑8 validada
- [x] Windows input — `_getch()` y bloqueo interactivo simple
- [x] Emisión modos — classic/unity/hybrid en ejemplo dedicado
- [x] Estáticos y namespaces — `Utils.Version.current()`
- [x] Herencia — `Child : Base` y llamada `Base.say()`
- [x] Native multi-line — triple quotes con varias líneas
- [x] Call definido — `call mundo.upp` resuelto en clase

## Pruebas y Calidad
- [ ] Tests en `cargo test` para `FileCall` y auto‑includes
- [ ] Pruebas de unicode adicionales (acentos, emojis)
- [ ] Reporte de build y tiempos (`--bench`) y validación de salida

## Documentación
- [x] README actualizado (EN)
- [ ] Ultra Language Manifesto (1 página)
- [ ] Guía de “Call” y mejores prácticas de emisión
- [ ] Guía de Emisión (`--emit classic|unity|hybrid`) con escenarios recomendados

## Guía de Uso Rápido (ampliada)
- [x] Watch — `ultracpp hello.upp dist --watch` (recarga en archivos individuales) ([main.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra C++/src/main.rs#L744-L763))
- [x] Compilador — `--compiler cl|g++|clang++` ([main.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra C++/src/main.rs#L855-L938))
- [x] Estándar — `--std c++17|c++20` ([main.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra C++/src/main.rs#L250-L254))
- [x] Emit — `--emit classic|unity|hybrid` ([main.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra C++/src/main.rs#L263-L287))
- [x] Convenience Windows — `.\ultra.bat combine` ([ultra.bat](file:///c:/Users/andre/OneDrive/Documentos/Ultra C++/ultra.bat))
- [x] Release — `--release` flag y perfiles Debug/Release
- [x] Calidad — `--lint`, `--format`, `--lint-rust`, `--smoke`, `--smoke-compilers` ([main.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra C++/src/main.rs))
- [x] Sanitizers — `--sanitize asan|ubsan|tsan` (según compilador) ([tool_detector.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra C++/src/tool_detector.rs))
- [x] Cobertura — `--coverage` (POSIX: g++/clang++; reporte con gcov/llvm-cov) ([tool_detector.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra C++/src/tool_detector.rs))
- [ ] GPU — `--gpu cuda|hip|oneapi|opencl|vulkan|dx12|metal` (según plataforma)
- [x] Puentes — `--bridge python|node|csharp|rust|java|otros` (genera bindings)

## Consideraciones adicionales (assets/config)
- Validar rangos de `width/height` y posición inicial del jugador
- Recargar `config.toml` con conversión horaria compatible MSVC
- Limitar longitud de sprites ASCII a un caracter visible
- Reaplicar `audio_map` en hot‑reload y emitir `config:reload`

## Bases de C++ y Compatibilidad (ampliación)
- [x] Const‑correctness en headers generados (evitar copias y mutación accidental)
- [x] RAII consistente: no usar `new/delete` en código generado (preferir objetos automáticos)
- [ ] Smart pointers en API pública opcional (`std::unique_ptr`/`std::shared_ptr` donde aplique)
- [x] Warnings‑as‑errors: `/W4 /WX` (MSVC), `-Wall -Wextra -Werror` (g++/clang++)
- [x] Conformidad estricta ISO: `/permissive-` (MSVC) y flags de estandar `c++17|c++20|c++23`
- [x] Minimizar includes en headers (forward declarations cuando sea posible)
- [x] Determinismo en codegen: orden estable de clases/namespaces e includes
- [x] Control de visibilidad de símbolos (`__declspec(dllexport/import)`, `-fvisibility=hidden`)
- [x] Política de excepciones: decidir y aplicar (excepciones vs códigos de error) en API
- [x] Localización/Unicode: política de encoding multiplataforma (Windows/Linux/macOS)
- [ ] Compatibilidad multiplataforma: MSVC/g++/clang++ con matrices de CI completas
- [x] Precompiled headers para `classic/hybrid` (mejorar tiempos de compilación)
- [x] Emisión CMake opcional (`--emit cmake`) para integración con ecosistema C++
- [ ] Integración con gestores de paquetes (vcpkg/conan) para dependencias futuras

## Extensiones y Facilidad de Uso
- [ ] `ultracpp init --template game` con plantillas ampliadas (loop, escenas, recursos)
- [ ] CLI avanzado: `--bench` con reporte de parse/codegen/compile/run por modo
- [ ] `--staging`/`--profile` para perfiles de ejecución y trazas de rendimiento
- [ ] Live‑reload de strings/audio/assets además de `config.toml`
- [ ] Sistema de plugins: carga dinámica (`LoadLibrary/dlopen`) para módulos de juego
- [ ] Modo “preview” Web (puente Node/Python) para visualización rápida

## Calidad y Herramientas
- [x] Sanitizers: ASan/UBSan/TSan según compilador (flags y runners)
- [x] clang‑tidy configurable y runner (`--lint`) sobre `dist/*`
- [x] Formateo: `clang-format` reglas y comando (`--format`)
- [x] Análisis estático en Rust: `cargo clippy` y `rustfmt` integrados en CI
- [x] Reporte de cobertura y smoke tests cruzados (classic/unity/hybrid)

## Backlog Priorizado (Acción)
Prioriza los próximos avances para ejecución continua.

### Alta
- [x] Preparar CI (GitHub Actions): matrix MSVC/g++/clang++ en Windows/Linux/macOS (build, `--smoke`, `--lint`, `--format`, `--lint-rust`)
- [ ] Cobertura en CI (Linux/macOS): publicar reporte con `gcov`/`llvm-cov`
- [ ] Tests en `cargo test`: `FileCall` y auto-includes
- [x] Smoke tests por compilador (MSVC/g++/clang++) con modos classic/unity/hybrid
- [ ] Seguridad: sanitización de inputs de config y paths

### Media
- [ ] GPU: flag `--gpu`, detección de toolchains y build por backend
- [ ] Demos de compute (vector_add/saxpy) con validación de resultados y tiempos
- [ ] Puentes: ejemplos empaquetables (Python/Node/C#/Rust/Java) y smoke en CI
- [ ] Packaging: inclusión de binarios `build/bin/*.exe` y assets mínimos

### Baja
- [ ] Documentación de “Call” y mejores prácticas de emisión
- [ ] Guía de escenarios `--emit` con recomendaciones detalladas
- [ ] Firmas y checksums para artefactos
- [ ] Manifesto de lenguaje y guía de puentes/ABI C

## Portabilidad y Multi‑arquitectura
- [ ] Builds x86/x64/ARM (Windows/Linux/macOS) con flags y toolchains adecuados
- [ ] Consideraciones de endianness, tamaño de tipos y `char` firmado/no firmado
- [ ] Paths y separadores multiplataforma, permisos y rutas relativas/absolutas seguras
- [ ] Reloj/tiempo portable (conversión de `file_time` → `system_clock` validada)

## Interoperabilidad avanzada
- [ ] ABI C estable con versionado semántico y pruebas de compatibilidad binaria
- [ ] Auto‑generación de bindings más completos (Python/Node/C#/Rust/Java)
- [ ] Ejemplos empaquetables y tests de bridges en CI (import/llamadas básicas)
- [ ] Políticas de memoria en FFI (ownership, lifetime, strings y buffers)

## Pruebas y CI ampliadas
- [ ] Golden tests de parser/codegen (snapshots) para sintaxis y emission
- [ ] Smoke tests por modo (classic/unity/hybrid) y por compilador (MSVC/g++/clang++)
- [ ] Tests de hot‑reload (cambios de config y assets) en Windows/Linux
- [ ] Pruebas de EventBus (suscripción/emit/prioridades y reentrancia)
- [ ] Benchmarks micro (render ASCII, update loop, carga de resources)

## Seguridad
- [ ] Sanitización de inputs de config y paths (prevención de traversal/inyección)
- [ ] Aislar límites de FFI/Puentes y validar payloads
- [ ] Política de logs (no exponer secretos, niveles y categorías)
- [ ] Opcional: sandbox de ejecución/lectura de archivos de demo

## Uso Práctico: Tareas Pendientes
- [x] Añadir perfiles de compilación (Debug/Release) con optimizaciones `/O2` (MSVC) y `-O2` (g++/clang++) en scripts de build ([tool_detector.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra C++/src/tool_detector.rs#L91-L131))
- [x] Exponer flag `--release` en CLI y reflejarlo en `build.bat`/`build.sh`
- [x] Documentar y probar `--watch` en modo carpeta `combine` (cambio de múltiples `.upp`)
- [ ] Preparar CI (GitHub Actions) con matrix MSVC/g++/clang++ para Windows/Linux/macOS (compilación y ejecución básica de demos)
- [ ] Documentar `ultracpp init <filename> --template game` como inicio rápido (plantilla juego) ([main.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra C++/src/main.rs#L162-L228))
- [x] Añadir guía de escenarios de emisión: cuándo usar `classic`, `unity` y `hybrid`
- [x] Ampliar diagnósticos de `call <file>.upp` con sugerencias y ejemplos cuando no se resuelva ([main.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra C++/src/main.rs#L69-L81))
- [ ] Publicar demos empaquetables (zip) con `build/bin/*.exe` y assets mínimos
- [x] Incluir reporte opcional de rendimiento `--bench`/`--staging` en README de salida de `dist` ([main.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra C++/src/main.rs#L640-L683))
 - [x] Añadir ejemplos de bridges (mock): Python y Node.js invocando `principal_run`/`hola_greet`

## Compatibilidad GPU (AMD/NVIDIA/Intel)
- [ ] Flag `--gpu cuda|hip|oneapi|opencl|vulkan|dx12|metal` en CLI ([main.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra C++/src/main.rs))
- [ ] Detección automática de toolchains: `nvcc`, `hipcc` (ROCm), `dpcpp/icpx` (oneAPI), `cl/g++` con OpenCL ([tool_detector.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra C++/src/tool_detector.rs))
- [ ] Integración en `build.bat`/`build.sh` para compilar kernels por backend (Windows/Linux/macOS)
- [ ] Abstracción de kernels: API común para lanzar y sincronizar (memoria unificada/copies)
- [ ] Demos de compute: `vector_add`, `saxpy`, con validación de resultados y tiempos
- [ ] Fallbacks claros cuando el driver/hardware no está disponible (mensajes y rutas alternativas CPU)
- [ ] Documentar diferencias por SO (ROCm mayormente Linux; oneAPI Windows/Linux/macOS)
- [ ] Windows: soporte DirectX 12 Compute (`dxc`, HLSL) para NVIDIA/AMD/Intel
- [ ] macOS: soporte Metal (MTL) cuando sea viable (Apple Silicon)

## Interoperabilidad (TODAS las lenguajes — vía puentes)
- [x] ABI C estable (esqueleto) — genera `exports.hpp`/`exports.cpp` con `extern "C"`
- [ ] Puentes:
  - [ ] Python (ctypes/cffi) — empaquetar `.dll/.so/.dylib` y ejemplo de llamada
  - [ ] Node.js (N-API) — módulo nativo y ejemplo `require()`/`import`
  - [ ] C# (P/Invoke) — ejemplo `DllImport` y llamada a funciones Ultra
  - [ ] Rust (bindgen) — crate con bindings y ejemplo de uso
  - [ ] Java (JNI) — `.jar` con native methods y ejemplo
- [x] Flag `--bridge python|node|csharp|rust|java` (parseo y generación de esqueleto de exports)
- [x] Scripts de build generando librerías compartidas (`.dll/.so/.dylib`) y headers públicos
- [ ] Demos: invocar `Hola.greet()` y `Principal.run()` desde cada lenguaje
- [ ] Documentación de uso y versionado semántico de puentes (compatibilidad estable)
- [ ] Otros lenguajes vía ABI C (Go, Swift/Kotlin Native, Zig, Ruby) — ejemplos
- [ ] Publicar paquetes de ejemplo (pip/npm/nuget/crates/maven) con mínima fricción

## Packaging y Distribución
- [x] Artefactos Release por plataforma (Windows `.zip`, Linux `.tar.gz`, macOS `.zip`)
- [ ] Inclusión de `build/bin/*.exe`/binarios, `include/*.hpp` y `README` de uso
- [ ] Firmas opcionales y checksums para integridad
- [x] Scripts de empaquetado automáticos tras `--release`

## Diagnósticos y Pruebas Adicionales
- [ ] Pruebas GPU por backend (CUDA/HIP/oneAPI/OpenCL/Vulkan/DX12/Metal) con validación y benchmarks
- [ ] Pruebas de puentes: integración y smoke tests en CI (Python/Node/C#/Rust/Java)
- [ ] Mensajes de error mejorados: detección de driver/faltan toolchains y sugerencias de instalación
- [ ] Reportes `--bench` con métricas clave (parse/codegen/compile/run) por modo y backend
