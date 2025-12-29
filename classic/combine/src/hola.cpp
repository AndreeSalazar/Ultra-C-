#include "hola.hpp"
#include <string>
#include <vector>
#include <iostream>
namespace Juego {
Hola::Hola() : name(std::string()) {}
Hola::Hola(std::string name) : name(name) {
}
std::string Hola::greet() {
  return std::string("Hola ") + name;
}
}
