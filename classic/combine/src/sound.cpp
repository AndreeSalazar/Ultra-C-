#include "sound.hpp"
#include <string>
#include <vector>
#include <iostream>
namespace Juego {
void Sound::play(std::string category, int priority, std::string message) {
  std::cout << "[SND][" << category << "][prio=" << priority << "] " << message << std::endl;
}
}
