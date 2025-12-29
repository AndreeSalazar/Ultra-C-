#pragma once
#include <string>
#include <vector>
#include <iostream>
#include <memory>
#include <string>
#include <memory>
#include <iostream>
#include <cmath>
#include "object.hpp"
class Entity : public Object {
public:
  float x;
  float y;
  std::string symbol;
  bool active;
  Entity();
  Entity(float x, float y);
  void render();
};
