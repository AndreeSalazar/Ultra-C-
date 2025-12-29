# Roadmap de Ultra C++ 🚀

Este documento detalla el plan de desarrollo y el estado actual del proyecto Ultra C++.

## Leyenda de Estado
- ✅ Completado: funcionalidad implementada y verificada.
- 🚧 En Progreso: desarrollo activo o parcial.
- 📅 Planificado: próximos pasos definidos.
- 🔮 Futuro: ideas a largo plazo.

## Fase 1: Fundamentos del Lenguaje (C++ Base)
Objetivo: cubrir sintaxis esencial y mapeo limpio a C++17.

- [x] ✅ Tipos Primitivos (`Int`, `Float`, `Bool`, `String`, `Void`)
- [x] ✅ POO
  - [x] ✅ Clases y Objetos
  - [x] ✅ Herencia `class Hijo(Base)` y `class Hijo : Base`
  - [x] ✅ Constructores `__init__`
  - [x] ✅ Métodos de Instancia (`self`)
  - [x] ✅ Métodos Estáticos (sin `self`)
  - [x] ✅ Encapsulamiento (`public`, `private`)
- [x] ✅ Control de Flujo
  - [x] ✅ `if / elif / else`
  - [x] ✅ `while`
  - [x] ✅ `return`
- [x] ✅ Operadores y Expresiones
  - [x] ✅ Aritmética (`+`, `-`, `*`, `/`)
  - [x] ✅ Comparación (`==`, `!=`, `<`, `>`, `<=`, `>=`)
  - [x] ✅ Operadores legibles: `and`, `or`, `not`
  - [x] ✅ Declaraciones: `let x: T = v` y `x: T`
- [x] ✅ Interoperabilidad Nativa
  - [x] ✅ `native "..."` (una y múltiples líneas)
  - [x] ✅ `native """ ... """` (triple‑quoted)
  - [x] ✅ Inclusiones estándar automáticas (`string`, `iostream`, `vector`)
  - [x] ✅ Inclusión condicional de `conio.h`/`windows.h` por uso de `_kbhit`, `_getch`, `Sleep`
- [x] ✅ Llamadas especiales
  - [x] ✅ `self.m()` → `this->m()`
  - [x] ✅ `super().m()` → `Base::m()`
  - [x] ✅ Dotted static: `Version.version()` → `Version::version()`

## Fase 2: Ecosistema y Herramientas
Objetivo: entorno de desarrollo robusto y amigable.

- [x] ✅ Transpilador Core (Rust)
  - [x] ✅ Parsing eficiente
  - [x] ✅ Generación de C++17 limpio
  - [x] ✅ Directivas (`profile`, `capability`, `use`, `entry`/`run`)
- [x] ✅ Gestión de Proyectos
  - [x] ✅ Estructura de directorios (`src`, `include`, `build/bin`, `build/obj`)
  - [x] ✅ Build scripts (`build.bat`, `build.sh`)
  - [x] ✅ Detección de compiladores (MSVC, Clang, G++)
  - [x] ✅ Plantillas de inicio (`ultracpp init --template game`)
- [x] ✅ Combine multi‑archivo
  - [x] ✅ Importación de `.upp` (prioriza `main.upp` ante conflictos)
  - [x] ✅ `entry.cpp` generado apuntando al método de entrada
  - [x] ✅ Auto‑includes por tipos en campos, parámetros y variables locales
- [x] ✅ Filecall directo
  - [x] ✅ `hola.upp()` imprime “Hola mundo”
  - [x] ✅ `<otro>.upp()` imprime “Run <otro>.upp”
- [x] ✅ Reportes de staging/bench
  - [x] ✅ Métricas de parseo y codegen
  - [x] ✅ Conteo de clases/campos/métodos
- [x] ✅ Sistema de Módulos
  - [x] ✅ Importación básica de archivos `.upp`
  - [x] ✅ Gestión de dependencias externas (versionado, cache, resolución)
  - [x] ✅ Namespaces y visibilidad entre módulos
- [ ] 🚧 Calidad y DX
  - [x] ✅ Type checker y resolución de símbolos (errores tempranos)
  - [x] ✅ Mejoras de diagnósticos (mensajes útiles, ubicaciones precisas)
  - [x] ✅ Integración CI (cargo clippy, pruebas, binarios)
  - [ ] 📅 Extensión para VS Code (syntax highlight, transpile on save)

## Fase 3: Computación de Alto Rendimiento (HPC) & GPU
Objetivo: integración progresiva de aceleración por hardware.

- [ ] 🔮 CUDA (NVIDIA)
  - [ ] Generación de kernels `.cu` desde `.upp`
  - [ ] Interfaz de memoria unificada y streams
- [ ] 🔮 ROCm (AMD)
  - [ ] Compatibilidad con HIP
- [ ] 🔮 Intel OneAPI
  - [ ] Integración con SYCL/DPC++
- [ ] 🔮 Backend HIP‑CPU
  - [ ] Paralelismo multi‑core optimizado

## Fase 4: Biblioteca Estándar Ultra (UltraStd)
Objetivo: abstracciones de alto nivel y utilidades prácticas.

- [ ] 📅 Matemáticas/Física (`Vector2`, `Vector3`, `Matrix`, `Transform`)
- [ ] 📅 E/S (FS simplificado, streams)
- [ ] 📅 Redes (Sockets, HTTP básico)
- [ ] 📅 Contenedores y utilidades (Lista, Mapa, Opcional)

## Fase 5: Lanzamiento y Distribución
Objetivo: empaquetado, distribución y documentación.

- [ ] 📅 Publicación de binarios (Win/Linux/macOS)
- [ ] 📅 Documentación y guías de usuario
- [ ] 📅 Ejemplos canónicos (Game, CLI, Utils)

---
Última actualización: 2025
