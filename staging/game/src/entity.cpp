#include "entity.hpp"
#include <string>
#include <vector>
#include <iostream>
Entity::Entity() : x(0.0f), y(0.0f), symbol(std::string()), active(false) {}
Entity::Entity(float x, float y) : x(x), y(y), symbol(std::string()), active(false) {
  this->x = x;
  this->y = y;
  this->symbol = std::string("?");
  this->active = true;
}
void Entity::render() {
  if(this->active) std::cout << this->symbol;
}
