#include "principal.hpp"
#include <iostream>
#ifdef _WIN32
#include <windows.h>
#endif
int main() {
  #ifdef _WIN32
    SetConsoleOutputCP(65001);
  #endif
  Juego::Principal obj(false, 0, 0);
  obj.hola_upp();
  return 0;
}
