# Roadmap de Ultra C++ 🚀

Este documento detalla el plan de desarrollo y el estado actual del proyecto Ultra C++.

## Leyenda de Estado
- ✅ **Completado**: Funcionalidad implementada y verificada.
- 🚧 **En Progreso**: Desarrollo activo o parcial.
- 📅 **Planificado**: Próximos pasos definidos.
- 🔮 **Futuro**: Ideas a largo plazo.

## Fase 1: Fundamentos del Lenguaje (C++ Base)
Objetivo: Soportar la totalidad de la sintaxis y características esenciales de C++ moderno.

- [x] ✅ **Tipos de Datos Primitivos** (`Int`, `Float`, `Bool`, `String`, `Void`)
- [x] ✅ **Programación Orientada a Objetos**
    - [x] ✅ Clases y Objetos
    - [x] ✅ Herencia (`class Child(Parent)`)
    - [x] ✅ Constructores (`__init__`)
    - [x] ✅ Métodos de Instancia (`self`)
    - [x] ✅ Métodos Estáticos (sin `self`)
    - [x] ✅ Encapsulamiento (`public`, `private`)
- [x] ✅ **Control de Flujo**
    - [x] ✅ Condicionales (`if`, `else`)
    - [x] ✅ Bucles (`while`)
    - [x] ✅ Retorno de valores (`return`)
- [x] ✅ **Operadores y Expresiones**
    - [x] ✅ Aritmética Básica (`+`, `-`, `*`, `/`)
    - [x] ✅ Comparación (`==`, `!=`, `<`, `>`, `<=`, `>=`)
    - [x] ✅ Asignación de Variables
- [x] ✅ **Interoperabilidad Nativa**
    - [x] ✅ Bloques `native "..."` (una y múltiples líneas)
    - [x] ✅ Inclusión automática de librerías estándar (`vector`, `string`, `iostream`)

## Fase 2: Ecosistema y Herramientas
Objetivo: Crear un entorno de desarrollo robusto y amigable.

- [x] ✅ **Transpilador Core** (Rust)
    - [x] ✅ Parsing eficiente
    - [x] ✅ Generación de código C++17 limpio
    - [x] ✅ Sistema de Directivas (`capability`, `profile`)
- [x] ✅ **Gestión de Proyectos**
    - [x] ✅ Estructura de directorios (`src`, `include`, `build`)
    - [x] ✅ Generación de Build Scripts (`build.bat`, `build.sh`)
    - [x] ✅ Detección automática de compiladores (MSVC, Clang, G++)
- [ ] 🚧 **Sistema de Módulos**
    - [ ] 📅 Importación de archivos `.upp`
    - [ ] 📅 Gestión de dependencias externas

## Fase 3: Computación de Alto Rendimiento (HPC) & GPU
Objetivo: Integrar soporte nativo para aceleración por hardware.

- [ ] 🔮 **Soporte CUDA (NVIDIA)**
    - [ ] Generación de kernels `.cu` desde `.upp`
    - [ ] Abstracción de memoria unificada
- [ ] 🔮 **Soporte ROCm (AMD)**
    - [ ] Compatibilidad con HIP
- [ ] 🔮 **Soporte Intel OneAPI**
    - [ ] Integración con SYCL/DPC++
- [ ] 🔮 **Backend HIP-CPU**
    - [ ] Paralelismo en CPU multi-core optimizado

## Fase 4: Biblioteca Estándar Ultra (UltraStd)
Objetivo: Proveer abstracciones de alto nivel para tareas comunes.

- [ ] 📅 **Matemáticas y Física** (`Vector2`, `Vector3`, `Matrix`)
- [ ] 📅 **Entrada/Salida** (Sistema de archivos simplificado)
- [ ] 📅 **Redes** (Sockets, HTTP básico)

---
*Última actualización: 2025*
