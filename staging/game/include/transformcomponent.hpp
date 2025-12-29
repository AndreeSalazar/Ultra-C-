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
class TransformComponent : public Object {
public:
  float x;
  float y;
  TransformComponent();
  TransformComponent(float x, float y);
};
