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
class Rect {
public:
  float x;
  float y;
  float w;
  float h;
  Rect();
  Rect(float x, float y, float w, float h);
  bool collides(Rect other) const;
};
}
