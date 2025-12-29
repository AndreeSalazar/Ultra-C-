#include "pch.hpp"
#include "sound.hpp"
#include <string>
#include <vector>
#include <iostream>
namespace Juego {
void Sound::play(const std::string& category, int priority, const std::string& message) {
  std::cout << "[SND][" << category << "][prio=" << priority << "] " << message << std::endl;
}
}
