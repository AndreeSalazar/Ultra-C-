# Ultra C++

![License](https://img.shields.io/badge/license-MIT-blue.svg) ![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg) ![Status](https://img.shields.io/badge/status-Active-green.svg)

> Copyright (c) 2025 Eddi André Salazar Matos — Peru 🇵🇪

---

## Purpose

Ultra C++ is a didactic, transpiled language that maps a clean, Python‑inspired syntax to modern, optimized C++17. It helps learners and professionals focus on algorithms and architecture while retaining full control over native C++ when needed.

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
2. Transpile to C++ (`.hpp` + `.cpp`)
3. Compile and run

### Minimal Example

```python
entry Main

class Main:
  def run():
    native "std::cout << \"Hello from Ultra C++!\" << std::endl;"
```

Transpile and compile:

```bash
ultracpp hello_world.upp dist
```

Project layout:
- `dist/include/`: generated `.hpp`
- `dist/src/`: generated `.cpp`
- `dist/build/bin`: final executable

## Features & Syntax

- Variables:
  - `let a: Int = 1`
  - `b: Float`
  - `c: String = "text"`
- Readable operators:
  - `and` → `&&`, `or` → `||`, `not` → `!`
- Control flow:
  - `if / elif / else` with indentation
- Native code:
  - `native """ ... """` for multi‑line C++
- Inheritance:
  - `class Child(Base):` or `class Child : Base:` (both supported)
- Self/Super calls:
  - `self.m()` → `this->m()`
  - `super().m()` → `Base::m()`
- Dotted static:
  - `Version.version()` → `Version::version()`
- Conditional Windows headers:
  - `conio.h` / `windows.h` are included only if native blocks use `_kbhit`, `_getch` or `Sleep`
- Auto‑includes:
  - Headers inferred from types in fields, params, and local declarations

## Multi‑File Combine

Ultra C++ can merge multiple `.upp` files in a folder, prioritizing `main.upp` for conflicts. Run:

```bash
ultracpp combine dist
```

Example `combine/main.upp`:

```python
profile std
profile math
capability io
entry Main
import hola.upp

class Main:
  def run():
    let msg: String = "Bundle OK"
    native "std::cout << msg << std::endl;"
```

## Quick Filecall

For rapid validation, you can call a `.upp` file directly:

```python
class Main:
  def run():
    hola.upp()
```

Transpiles to:

```cpp
std::cout << "Hola mundo" << std::endl;
```

Other file names produce:

```cpp
std::cout << "Run <name>.upp" << std::endl;
```

## Entry Generation

When `entry Main` or `run Main` is present, an `entry.cpp` is generated that calls `Main::run()` or similar entry points (`run_loop`, `start`, `main`, `run`).

## Testing

Run the Rust test suite:

```bash
cargo test
```

Includes tests for directives, syntax keywords, `elif`, inheritance (both styles), native triple quotes, Windows header detection, declarations, and filecall behavior.

## Roadmap

See [ROADMAP.md](./ROADMAP.md) for planned support (GPU compute, extended stdlib, advanced optimizations).

## Contributing

Community contributions are welcome under the MIT license. If you share the vision of accessible high‑performance education, join in.
