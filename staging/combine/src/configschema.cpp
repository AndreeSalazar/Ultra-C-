#include "configschema.hpp"
#include "pch.hpp"
#include <string>
#include <vector>
#include <iostream>
namespace Juego {
bool ConfigSchema::validate(const Config& cfg) {
  if (cfg.width < 10 || cfg.width > 100) { std::cerr << "config width out of range" << std::endl; return false; }
  if (cfg.height < 5 || cfg.height > 60) { std::cerr << "config height out of range" << std::endl; return false; }
  if (cfg.start_x < 0 || cfg.start_x >= cfg.width || cfg.start_y < 0 || cfg.start_y >= cfg.height) { std::cerr << "player start out of bounds" << std::endl; return false; }
  return true;
}
}
