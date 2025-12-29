#include "rect.hpp"
#include <string>
#include <vector>
#include <iostream>
namespace Juego {
Rect::Rect() : x(0.0f), y(0.0f), w(0.0f), h(0.0f) {}
Rect::Rect(float x, float y, float w, float h) : x(x), y(y), w(w), h(h) {
}
bool Rect::collides(Rect other) const {
  return (x < other.x + other.w) && (x + w > other.x) && (y < other.y + other.h) && (y + h > other.y);
}
}
