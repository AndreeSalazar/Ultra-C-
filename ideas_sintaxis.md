# Ideas de Sintaxis Ultra C++ (2025)

- Importación directa en main.upp:
  - `import hola.upp` integra clases del archivo en el compilado actual, priorizando definiciones locales.
- Herencia flexible:
  - `class Hijo(Base):` y `class Hijo : Base:` son equivalentes.
- Variables simples y expresivas:
  - `let a: Int = 1`
  - `let b: Float`
  - `c: String = "texto"`
- Operadores legibles:
  - `and`, `or`, `not` mapean a `&&`, `||`, `!`.
- Control de flujo:
  - `if / elif / else` con indentación, transpila a `if / else if / else`.
- Nativo multilinea:
  - `native """ ... """` para bloques largos.
- Llamadas especiales:
  - `self.m()` → `this->m()`
  - `super().m()` → `Base::m()`
20→- Dotted static:
21→  - `Version.version()` → `Version::version()`.
22→- Futuras extensiones:
23→  - `for item in vector:` con soporte de perfiles `std::vector`.
24→  - `switch x:` con casos simples, mapeo a `switch` C++ donde aplique.
25→  - `trait`/`interface` sintácticos para patrones de diseño.
26→- Filecall directo:
27→  - `hola.upp()` como llamada básica de archivo. En ejemplos iniciales imprime "Hola mundo" para validar integración rápida.
