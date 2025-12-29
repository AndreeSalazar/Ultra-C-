#include "entity.hpp"
#include <string>
#include <vector>
#include <iostream>
namespace Juego {
Entity::Entity() : x(0.0f), y(0.0f) {}
Entity::Entity(float x, float y) : x(x), y(y) {
}
void Entity::move(float dx, float dy) {
  x += dx; y += dy;
}
}
