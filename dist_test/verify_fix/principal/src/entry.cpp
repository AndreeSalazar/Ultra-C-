#include "pch.hpp"
#include "principal.hpp"
#include <iostream>
#ifdef _WIN32
#include <windows.h>
#endif
int main() {
  #ifdef _WIN32
    SetConsoleOutputCP(65001);
  #endif
  Juego::Principal obj(false, 0, 0, false, 0, 0, 0, 0, std::string("Mundo"), {}, {}, {}, {}, std::string("Mundo"), std::string("Mundo"), {});
  obj.run_loop();
  return 0;
}
