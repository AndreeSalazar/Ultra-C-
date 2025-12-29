#include "principal.hpp"
#include "hola.hpp"
#include "version.hpp"
#include <string>
#include <vector>
#include <iostream>
namespace Juego {
Principal::Principal() : running(false), score(0), level(0) {}
Principal::Principal(bool running, int score, int level) : running(running), score(score), level(level) {
}
void Principal::hola_upp() {
  std::cout << "Versión actual: " + Utils::Version::current() << std::endl;
  auto h = Hola("como están");
  std::cout << h.greet() << std::endl;
}
void Principal::start() {
  std::cout << "--- Start ---" << std::endl;
  hola_upp();
}
void Principal::loop() {
  std::cout << "--- Loop Tick ---" << std::endl;
  hola_upp();
}
void Principal::run() {
  std::cout << "--- Iniciando Ejecución ---" << std::endl;
  start();
  loop();
  std::cout << "--- Fin Ejecución ---" << std::endl;
}
}
