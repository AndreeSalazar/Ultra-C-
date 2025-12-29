# Guía de Desarrollo de Juegos con Ultra C++

Esta guía detalla el proceso de creación, generación y compilación de juegos utilizando el motor Ultra C++.

## 1. Inicialización del Proyecto

Ultra C++ incluye un generador de plantillas para facilitar el inicio del desarrollo.

### Comando Init
Para crear un nuevo archivo de definición de juego (`.upp`) con la estructura básica:

```bash
cargo run -- init game.upp --template game
```

O si tienes el binario compilado en tu PATH:
```bash
ultracpp init game.upp --template game
```

Esto generará un archivo `game.upp` que incluye:
- **Directivas Globales**: Configuración base del motor.
- **Clases Base**: `Entity` para objetos del juego.
- **Clase Principal**: `Game` con el bucle principal (`start`, `loop`).
- **Punto de Entrada**: Instrucción `run Game`.

## 2. Estructura del Archivo .upp

El archivo generado sigue esta estructura:

```python
global              # Habilita herencia de Object por defecto
profile math        # Incluye librerías matemáticas (cmath, vectores)
capability io       # Habilita entrada/salida (logs, archivos)

class Entity:
    x: Float
    y: Float
    def __init__(self, x: Float, y: Float):
        return ""

class Player(Entity):
    name: String
    def __init__(self, name: String):
        return super().__init__(0.0, 0.0)

class Game:
    running: Bool
    def start(self):
        # Inicialización
    def loop(self):
        # Lógica por frame

run Game
```

## 3. Generación de Código C++

Una vez definido el juego, el transpilador convierte el código Ultra C++ a C++ nativo optimizado.

### Comando de Construcción
```bash
cargo run -- game.upp dist
```

### Salida Generada (`dist/game/`)
El sistema genera los siguientes archivos en la carpeta de destino:

| Archivo | Descripción |
|---------|-------------|
| `main.cpp` | Punto de entrada (main) que instancia la clase Game. |
| `game.hpp/cpp` | Lógica principal del juego. |
| `entity.hpp/cpp` | Clase base para entidades. |
| `player.hpp/cpp` | Ejemplo de clase derivada. |
| `build.bat` | Script de compilación para Windows (MSVC/MinGW). |
| `build.sh` | Script de compilación para Unix (GCC/Clang). |

## 4. Compilación y Ejecución

Para compilar el código C++ generado:

**Windows:**
```cmd
cd dist/game
build.bat
```

**Linux/Mac:**
```bash
cd dist/game
./build.sh
```

Esto generará un ejecutable `game.exe` (o `game`) que puede ejecutarse directamente.

## 5. Consideraciones de Rendimiento

- **Gestión de Memoria**: Las clases generadas utilizan punteros inteligentes (`std::shared_ptr`) donde es apropiado gracias a la herencia de `Object`.
- **Inlining**: Métodos pequeños en `.upp` pueden ser candidatos a inlining en C++.
- **Vectores**: Al usar `profile math`, se optimizan las operaciones con `Float`.

## 6. Lista de Verificación (Checklist)

Consulte `CHECKLIST_GAME.md` para validar que su juego cumple con todos los requisitos antes del despliegue.
