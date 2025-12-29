#include "transformcomponent.hpp"
#include <string>
#include <vector>
#include <iostream>
TransformComponent::TransformComponent() : x(0.0f), y(0.0f) {}
TransformComponent::TransformComponent(float x, float y) : x(x), y(y) {
  this->x = x;
  this->y = y;
}
