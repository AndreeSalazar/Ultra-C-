#include "checklist.hpp"
#include <string>
#include <vector>
#include <iostream>
void Checklist::print_status() {
  
  std::cout << "\\n=== CHECKLIST DE CARACTERÍSTICAS ===\\n";
  std::cout << "[✔️] Configuración (Ventana/FPS)\\n";
  std::cout << "[✔️] Estados (Menu, Juego, Pausa, Fin)\\n";
  std::cout << "[✔️] Sistema de Entidades/Componentes\\n";
  std::cout << "[✔️] Input No Bloqueante (Win32)\\n";
  std::cout << "[✔️] Física y Colisiones AABB\\n";
  std::cout << "[✔️] Puntuación y Logs\\n";
  std::cout << "====================================\\n";
}
