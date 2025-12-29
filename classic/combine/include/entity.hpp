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
namespace Juego {
class Entity {
public:
  float x;
  float y;
  Entity();
  Entity(float x, float y);
  void move(float dx, float dy);
};
}
