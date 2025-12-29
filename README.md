# Ultra C++

![License](https://img.shields.io/badge/license-MIT-blue.svg) ![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg) ![Status](https://img.shields.io/badge/status-Active-green.svg)

> Copyright (c) 2025 Eddi André Salazar Matos — Peru 🇵🇪

---

## Purpose

Ultra C++ is a didactic, transpiled language that maps a clean, Python‑inspired syntax to modern C++17. It lets you focus on algorithms and architecture while keeping full, explicit C++ when needed. The generated C++ is readable and “enterprise‑friendly”: clear namespaces, explicit main, and optional unity builds.

## Installation

- Rust toolchain (to build the transpiler)
- C++ compiler: MSVC (Windows) or g++/clang++ (Linux/macOS)

Build from source:

```bash
git clone https://github.com/EddiAndre/UltraCpp.git
cd UltraCpp
cargo build --release
```

The binary will be at `target/release/ultracpp`.

## Workflow

1. Write `.upp` files (Ultra syntax)
2. Transpile to C++ (classic: `.hpp` + `.cpp`, or unity: `all.cpp`)
3. Compile and run (MSVC / g++ / clang++)

### Minimal Example (Ultra syntax)

```python
entry Main

class Main:
  run():
    print("Hello from Ultra C++!")
```

Transpile and compile (Windows):

```bash
.\ultra.bat hello_world.upp
```

Transpile and compile (Linux/macOS):

```bash
ultracpp hello_world.upp dist --compile
```

Project layout (classic):
- `dist/<project>/include/`: generated `.hpp`
- `dist/<project>/src/`: generated `.cpp` + `entry.cpp`
- `dist/<project>/build/bin`: final executable

## Features & Syntax

- Variables:
  - `a: Int = 1`
  - `b: Float`
  - `c: String = "text"`
- Readable operators:
  - `and` → `&&`, `or` → `||`, `not` → `!`
- Control flow:
  - `if / elif / else` with indentation
- Native code:
  - `native """ ... """` for multi‑line C++
- Inheritance:
  - `class Child(Base):` and `class Child : Base:` (both supported)
- Implicit self:
  - Instance calls don’t require `self`; Ultra generates idiomatic C++ (`m()`, no `this->` unless needed)
- Super calls:
  - `super().m()` → `Base::m()`
- Dotted statics and namespaces:
  - `Utils.Version.current()` → `Utils::Version::current()`
- Conditional Windows headers:
  - `conio.h` / `windows.h` are included only if native blocks use `_kbhit`, `_getch` or `Sleep`
- Auto‑includes:
  - Headers inferred from fields, params, local declarations, class references and static calls (e.g., `Utils::Version::...`)
- Printing:
  - `print(...)` → `std::cout << ... << std::endl`

## Multi‑File Combine

Ultra C++ can merge multiple `.upp` files in a folder, prioritizing `principal.upp` as the entry if present. Run:

```bash
ultracpp combine dist
# Windows convenience:
.\ultra.bat combine
# Watch changes in a folder (auto‑rebuild on .upp changes):
ultracpp combine dist --watch
```

Example `combine/principal.upp`:

```python
std
entry Principal
import hola.upp

class Principal:
  hola.upp():
    print("Version: " + Utils.Version.current())
    h := Hola("world")
    print(h.greet())

  run():
    print("--- Start ---")
    call hola.upp
    print("--- End ---")
```

## Filecall (`call`)

For explicit, readable dispatch you can call a local method mapped from a file name:

```python
class Principal:
  hola.upp():
    print("Hello")
  run():
    call hola.upp
```

Transpiles to:

```cpp
Principal::hola_upp();
```

Fallback (when no matching method is found):

```cpp
std::cout << "Run <name>.upp" << std::endl;
```

## Entry Generation

When `entry Main`/`entry Principal` is present, an `entry.cpp` is generated that calls the class entry method (`run_loop`, `start`, `main`, or `run`). On Windows, UTF‑8 output is enabled via `SetConsoleOutputCP(65001)`.

## Emission Modes
 
 - Classic (`--emit classic` or default via `ultra.bat`):
   - Generates headers + sources and a standalone `entry.cpp`.
 - Unity (`--emit unity`):
   - Generates a single `all.cpp` with explicit `main()`. Useful for CI, prototyping, and deterministic builds.
 - Hybrid (`--emit hybrid`):
   - Public headers + unity source internally. Available for projects that need public interfaces with fast unified compilation.
 
 ### Cuándo usar cada modo
 - Classic:
   - Proyectos con interfaces públicas claras (headers) y build tradicional.
   - Integración con librerías externas y empaquetado granular.
 - Unity:
   - Prototipos rápidos, CI determinista, builds sencillos en un solo archivo.
   - Menor tiempo de compilación en proyectos pequeños/medianos.
 - Hybrid:
   - APIs públicas en headers con build interno tipo unity para compilar rápido.
   - Útil cuando quieres distribuir headers pero mantener fuentes unificadas.

## Release Builds

Enable optimized builds:

```bash
# Windows
ultracpp combine dist --compile --release
# Linux/macOS
ultracpp combine dist --compile --release
```

Build scripts add `/O2` (MSVC) or `-O2` (g++/clang++) automatically.

## Testing

```bash
cargo test
```

Includes tests for directives, keywords, `elif`, inheritance (both styles), native triple quotes, Windows header detection, declarations, and filecall behavior.

## Roadmap

- Unified `--emit classic|unity|hybrid`
- Header include deduplication
- Extended stdlib and advanced optimizations
- GPU compute examples
- Better diagnostics for `call` resolution
- See [ROADMAP.md](./ROADMAP.md) for more

## Contributing

Community contributions are welcome under the MIT license. If you share the vision of accessible high‑performance education, join in.
