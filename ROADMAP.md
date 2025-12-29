# Roadmap Extendido: Ultra C++ (vNext)

Planificación ampliada y accionable para **heredar TODO C++** de forma segura, determinista y verificable, manteniendo una **UX mínima** (el usuario solo expresa diseño).

---

## Objetivo Macro

Construir un **traductor OOP-first** que genere **C++ moderno, auditable y portable**, con una **base global implícita** y **perfiles declarativos** para salir del default solo cuando sea necesario.

> **Contrato**: Ultra C++ nunca oculta C++; lo **normaliza** y **automatiza**.

---

## Estado Ejecutivo (Resumen)

* Núcleo OOP, generación `.hpp/.cpp/main.cpp`: **Listo**
* Base global implícita + `Object`: **Listo**
* Typesystem mínimo + includes automáticos: **Parcial**
* Herencia básica + `super()`: **Listo**
* Control de flujo + asignación: **Plan inmediato**
* Colecciones + genéricos: **Plan inmediato**
* Interop C++ + toolchain avanzado: **Plan medio**

---

## Arquitectura Definitiva por Capas

### Capa 0 — Base Global Implícita (Siempre Activa)

**Nunca se declara.**

* Runtime: `std::string`, `std::move`, `std::unique_ptr`, `std::shared_ptr`, `std::nullptr_t`, RAII
* Archivos: `.hpp`, `.cpp`, `main.cpp`
* Headers: `#pragma once`
* Defaults: `= default`, `noexcept` cuando aplique
* ABI-safe: sin macros, sin RTTI extra, sin GC

**Verificación**: headers compilables sin includes manuales.

---

### Capa 1 — Base `Object` (Herencia Global)

* Regla: clase sin base ⇒ `public Object`
* Contenido mínimo:

  ```cpp
  class Object { public: virtual ~Object() = default; };
  ```
* Generación automática: `object.hpp/.cpp`

**Verificación**: herencia visible y dtor virtual.

---

### Capa 2 — OOP Completo (Diseño Cerrado, Activación Gradual)

Preparar soporte interno para:

* `virtual`, `override`, `final`
* `protected` / `private`
* Métodos `const`
* Constructores copy/move
* Herencia múltiple (desactivada por defecto)

**Sintaxis futura (opcional)**:

```
Animal
  speak -> String virtual
```

**Verificación**: pruebas unitarias sin exponer flags al usuario.

---

### Capa 3 — Typesystem Determinista (Mapa Total)

**Primitivos**

* `Int → int`, `Float → float`, `Bool → bool`, `Char → char`

**Especiales**

* `String → std::string`
* `Array[T] → std::vector<T>`
* `Map[K,V] → std::unordered_map<K,V>`
* `Option[T] → std::optional<T>`
* `Ref[T] → T&`
* `Ptr[T] → std::unique_ptr<T>`

**Regla**: uso ⇒ include automático.

---

### Capa 4 — Librerías Estándar (Auto-Includes)

* Detección por AST/uso:

  * String ⇒ `<string>`
  * Array ⇒ `<vector>`
  * Map ⇒ `<unordered_map>`
  * Option ⇒ `<optional>`
  * IO ⇒ `<iostream>`
  * Memory ⇒ `<memory>`

**Nunca** se escriben includes en `.upp`.

---

### Capa 5 — Entry Point

```
run Hola
```

Genera:

```cpp
int main(){ Hola app{}; return 0; }
```

Extensiones planificadas:

* `run Hola(args)`
* `test Hola`

---

### Capa 6 — Build Knowledge (Toolchain)

Conocimiento interno:

* MSVC / Clang / GCC
* Flags seguros (`/std:c++17`, `-std=c++17`, `-Wall`)
* `--compiler`, `--std`, `--no-main`

**Artefactos**:

* `build.bat` (Windows)
* `build.sh` (Unix)

---

### Capa 7 — Perfiles Declarativos (Dormidos por Default)

Solo se usan cuando se necesita **salir del default**.

* `profile bare` (sin std)
* `profile engine` (math/simd)
* `profile net` (sockets)
* `profile gpu` (bindings)

Cada perfil define:

* includes
* aliases
* flags

---

### Capa 8 — Seguridad y Prevención

* ODR-safe
* Orden de includes
* Forward declarations automáticas
* Prohibición de `new/delete` manual
* Warnings limpios

---

### Capa 9 — Interop C++

* Incluir headers externos
* Link flags
* Namespaces
* Integración gradual sin migración total

---

## UX Final (Lo Único que Ve el Usuario)

```
Hola
  name String

  greet -> String
    "Hola " + name

run Hola
```

> Todo lo demás es responsabilidad de Ultra C++.

---

## Fases de Ejecución (Actualizadas)

### Fase A — Control de Flujo

* Asignación
* `if / while`
* Expresiones compuestas

### Fase B — Constructores

* `__init__`
* `super().__init__()`
* Listas de inicialización C++

### Fase C — Genéricos

* `Vector[T]`, `Map[K,V]`
* Iteración básica

### Fase D — Tooling

* Watch mode
* Logs de generación
* Debug de AST

---

## Métricas de Éxito

* 0 includes manuales
* 0 wiring humano
* 100% C++ auditable
* Compila en MSVC/GCC/Clang

---

## Entregables de Planificación

* Especificación Ultra C++ 1.0
* CLI estable
* Ejemplos `.upp`
* Roadmap versionado

---

## Manifiesto (1 línea)

**Ultra C++ convierte intención OOP mínima en C++ completo, seguro y humano.**

---

## Sintaxis `.upp` implementable ahora
 - Directivas soportadas:
   - `std` activa el perfil estándar (equivalente a `profile std`).
   - `global` activa herencia implícita de `Object` y el perfil estándar.
   - `run Nombre` o `entry Nombre` selecciona la clase para `main.cpp`.
   - `use X`, `profile X`, `capability X` activan includes según intención:
     - `use std`, `use std::io`, `use std::string`, `use std::vector`
     - `profile std`, `profile math`
     - `capability io`, `capability string`, `capability vector`
 - Clases (estilo compacto, con perfil estándar):
 
   ```
   std
 
   Hola
     name String
 
     greet -> String
       "Hola " + name
 
   run Hola
   ```
 
   - Campo: admite `name String` o `name: String`.
   - Método compacto: `greet -> Tipo` con cuerpo indentado en la siguiente línea.
 - Clases (estilo Python):
 
   ```
   class Hola:
       nombre: String
       def saludo(self) -> String:
           return "Hola " + self.nombre
   ```
 - Herencia (Python + `super()`):
 
   ```
   class Base:
       nombre: String
       def saludo(self) -> String:
           return self.nombre
   class Hijo(Base):
       apellido: String
       def saludo(self) -> String:
           return super().saludo() + " " + self.apellido
   ```
 - Múltiples clases por archivo:
   - Se admiten varias declaraciones en `.upp`; se generan `.hpp/.cpp` por clase.
 - Métodos estáticos:
 
   ```
   class Version:
       def version() -> String:
           return "1.0.0"
   ```
 - Literales y expresiones:
   - Soportados: `String`, `Int`, `Bool`, `Float`
   - Operaciones: `+` (concatenación/suma), `self.campo`, `self.metodo()`, `super().metodo()`
 - Compilación y ejecución:
   - Generar C++: `cargo run -- hola.upp dist --compile`
   - Salida en `dist/hola/`: `hola.hpp`, `hola.cpp`, `main.cpp`, `build.bat`
   - Compilar (MSVC): `cd dist/hola && .\build.bat`
   - Ejecutar: `.\hola.exe`
 - Checklist de verificación rápida:
   - ✅ `dist/<base>/` contiene `.hpp/.cpp/main.cpp/build.bat`
   - ✅ `main.cpp` instancia la clase seleccionada por `run/entry`
   - ✅ Includes estándar presentes sin escribirlos en `.upp`
   - ✅ Herencia de `Object` activa si `global` está en el `.upp`
   - ✅ `super()` se transpila a `Base::metodo(...)` cuando hay clase base
 
 Referencias útiles:
 - Parser actual: [parser.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra%20C++/src/parser.rs)
 - Generación C++: [codegen.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra%20C++/src/codegen.rs)
 - CLI/entry/includes: [main.rs](file:///c:/Users/andre/OneDrive/Documentos/Ultra%20C++/src/main.rs)
