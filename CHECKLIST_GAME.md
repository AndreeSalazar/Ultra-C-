# Checklist de Verificación: Desarrollo de Juegos en Ultra C++

## 1. Requisitos Funcionales
- [ ] **Bucle de Juego (Game Loop)**: Método `start()` y `loop()` definidos en la clase principal.
- [ ] **Entidades**: Clase base `Entity` con coordenadas (`x`, `y`) e identificador.
- [ ] **Jugador**: Clase `Player` que hereda de `Entity` con lógica de movimiento/estado.
- [ ] **Estado**: Variables de control (`running`, `score`, `level`).
- [ ] **Entrada/Salida**: Uso de `std::iostream` (vía perfil `io` o `std`) para simular renderizado y logs.

## 2. Assets Necesarios
- [ ] **Gráficos**: Representación ASCII en métodos `draw()` o `render()`.
- [ ] **Sonidos**: Logs de consola simulando eventos (`"Playing sound: jump.wav"`).
- [ ] **Configuración**: Valores iniciales en constructores (`__init__`).

## 3. Configuración del Motor (Ultra C++)
- [ ] **Directivas**:
  - `global`: Para herencia de `Object` y gestión de memoria.
  - `profile math`: Para cálculos de física/movimiento.
  - `capability io`: Para salida por consola.
- [ ] **Entry Point**: `run Game` al final del archivo.

## 4. Rendimiento y Optimización
- [ ] **Tipos de Datos**: Uso de `Int` y `Float` preferentemente sobre `String` para lógica.
- [ ] **Gestión de Memoria**: Ultra C++ usa `std::string` y valores en stack/heap automático; evitar copias innecesarias en bucles.
- [ ] **Compilación**: Usar flag `--release` en cargo (para el transpilador) y `-O2`/`/O2` en el compilador C++ (configurado en `build.bat` generado).
