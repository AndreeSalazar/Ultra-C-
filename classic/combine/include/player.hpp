#pragma once
#include <string>
#include <vector>
#include <iostream>
#include <memory>
#include <map>
#include <list>
#include <optional>
#include <thread>
#include <mutex>
#include <future>
#include <atomic>
#include <filesystem>
#include <fstream>
#include <algorithm>
#include <numeric>
#include <cmath>
#include <cstdio>
#include <functional>
#include "rect.hpp"
#include "entity.hpp"
namespace Juego {
class Player : public Entity {
public:
  std::string name;
  Player();
  Player(std::string name);
  void render();
  Rect get_rect() const;
};
}
