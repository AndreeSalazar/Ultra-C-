# Ultra C++

<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/c/cc/Escudo_nacional_del_Per%C3%BA.svg" alt="Escudo del Perú" width="100"/>
  <br>
  <em>Hacia la democratización del conocimiento tecnológico</em>
</div>

---

## Propósito del Proyecto

> "Esta iniciativa busca democratizar el acceso a C++ mediante herramientas pedagógicas que combinan rigor técnico con elegancia conceptual. Nuestro objetivo es transformar la complejidad inherente del lenguaje en una experiencia de aprendizaje fluida y accesible."

Ultra C++ nace como una respuesta a la barrera de entrada que a menudo representa C++ moderno. Al fusionar la sintaxis intuitiva de lenguajes de alto nivel con la potencia del metal desnudo de C++, facilitamos un viaje educativo desde la abstracción hasta la implementación concreta.

## Introducción

Ultra C++ es un transpilador vanguardista escrito en Rust, diseñado para convertir código con sintaxis orientada a objetos pura (similar a Python) en C++ moderno (C++17). No solo genera código; educa. Cada línea transpilada es un ejemplo de cómo se estructuran aplicaciones robustas, generando automáticamente archivos de cabecera (`.hpp`), fuentes (`.cpp`) y scripts de construcción multiplataforma.

## Instalación Detallada

Para integrar Ultra C++ en su flujo de trabajo, asegúrese de contar con un entorno preparado para la excelencia.

### Prerrequisitos
*   **Rust (Cargo)**: El corazón de nuestro transpilador. Asegúrese de tener la última versión estable.
*   **Compilador de C++**:
    *   *Windows*: Visual Studio Build Tools (recomendado para integración nativa) o MinGW-w64.
    *   *Linux/macOS*: GCC (`g++`) o LLVM (`clang++`).

### Proceso de Compilación
1.  Clone este repositorio o descargue el código fuente.
2.  Abra su terminal en la raíz del proyecto.
3.  Ejecute el comando de construcción optimizada:
    ```bash
    cargo build --release
    ```
4.  El ejecutable `ultracpp` estará disponible en `target/release/`.

## Ejemplos Paradigmáticos

A continuación, presentamos cómo Ultra C++ transforma conceptos abstractos en realidades tangibles.

### 1. El Saludo Clásico (Hola Mundo Orientado a Objetos)
Un ejemplo minimalista que ilustra la definición de clases, tipado fuerte y métodos.

**Archivo: `hola.upp`**
```python
class Hola:
    nombre: String

    def saludo(self) -> String:
        return "Saludos cordiales, " + self.nombre
```

**Generación:**
Ejecute el transpilador para observar la magia:
```bash
ultracpp hola.upp
```
Esto orquestará una estructura completa en `dist/hola/`, separando limpiamente la declaración (`hola.hpp`) de la implementación (`hola.cpp`).

## Buenas Prácticas Estilizadas

Para mantener la armonía entre el código fuente y el código generado, recomendamos:

*   **Tipado Explícito**: Ultra C++ valora la claridad. Declare siempre los tipos de sus campos y retornos (ej. `String`, `Int`, `Void`).
*   **Nomenclatura PascalCase**: Para las clases (ej. `GestorDeJuego`), evocando estructura y solidez.
*   **Indentación Consistente**: La estructura visual define la estructura lógica. Mantenga una indentación pulcra (4 espacios).
*   **Modularidad**: Divida su lógica en múltiples clases y archivos para facilitar la mantenibilidad y el estudio de componentes aislados.

## Roadmap de Desarrollo

Nuestro viaje hacia la excelencia continúa. Consulte [ROADMAP.md](ROADMAP.md) para una visión detallada. Hitos clave incluyen:
*   Soporte avanzado para plantillas (Templates).
*   Gestión de memoria inteligente y transparente.
*   Integración con bibliotecas gráficas para desarrollo lúdico.

---

## Licencia

**Copyright (c) 2025 Eddi André Salazar Matos - Perú**

Se concede permiso, de forma gratuita, a cualquier persona que obtenga una copia de este software y de los archivos de documentación asociados (el "Software"), para tratar con el Software sin restricciones, incluyendo, sin limitación, los derechos de uso, copia, modificación, fusión, publicación, distribución, sublicencia y/o venta de copias del Software, y para permitir a las personas a las que se les proporcione el Software a hacerlo, sujeto a las siguientes condiciones:

El aviso de copyright anterior y este aviso de permiso se incluirán en todas las copias o partes sustanciales del Software.

EL SOFTWARE SE PROPORCIONA "TAL CUAL", SIN GARANTÍA DE NINGÚN TIPO, EXPRESA O IMPLÍCITA, INCLUYENDO PERO NO LIMITADO A LAS GARANTÍAS DE COMERCIABILIDAD, IDONEIDAD PARA UN PROPÓSITO PARTICULAR Y NO INFRACCIÓN. EN NINGÚN CASO LOS AUTORES O TITULARES DEL COPYRIGHT SERÁN RESPONSABLES DE NINGUNA RECLAMACIÓN, DAÑO U OTRA RESPONSABILIDAD, YA SEA EN UNA ACCIÓN DE CONTRATO, AGRAVIO O DE OTRO TIPO, QUE SURJA DE, FUERA DE O EN CONEXIÓN CON EL SOFTWARE O EL USO U OTRAS TRATOS EN EL SOFTWARE.
