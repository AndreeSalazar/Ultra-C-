#include "game.hpp"
#include <string>
#include <vector>
#include <iostream>
#ifdef _WIN32
#include <conio.h>
#include <windows.h>
#endif
Game::Game() : running(false), state(0), width(0), height(0), frame_count(0), checklist({}) {
  
  this->running = true;
  this->state = 0; // Menu
  this->width = 20;
  this->height = 10;
  this->frame_count = 0;
}
void Game::clear_screen() {
  
  #ifdef _WIN32
  system("cls");
  #else
  system("clear");
  #endif
}
void Game::log(std::string msg) {
  std::cout << "[LOG] " << msg << std::endl;
}
void Game::check_collisions(Player p, Enemy e) {
  
  if (!e.active) return;
  float dx = p.x - e.x;
  float dy = p.y - e.y;
  if (std::abs(dx) < 0.5 && std::abs(dy) < 0.5) {
  this->log("¡Combate! Has recibido daño.");
  p.take_damage(e.damage);
  e.active = false; // Enemigo derrotado
  p.score += 50;
  }
}
void Game::draw_ui(Player p) {
  
  std::cout << " Ultra C++ Roguelike | Score: " << p.score << " | HP: " << p.hp << "/" << p.max_hp << "\\n";
  if (this->state == 3) std::cout << " [PAUSA] - Presiona P para continuar\\n";
}
void Game::draw_map(Player p, Enemy e) {
  this->clear_screen();
  this->draw_ui(p);
  
  std::cout << "+";
  for(int i=0; i<this->width; i++) std::cout << "-";
  std::cout << "+\\n";
  
  for(int y=0; y<this->height; y++) {
  std::cout << "|";
  for(int x=0; x<this->width; x++) {
  bool drawn = false;
  if (std::abs(x - (int)p.x) == 0 && std::abs(y - (int)p.y) == 0) {
  std::cout << p.symbol;
  drawn = true;
  } else if (e.active && std::abs(x - (int)e.x) == 0 && std::abs(y - (int)e.y) == 0) {
  std::cout << e.symbol;
  drawn = true;
  }
  
  if (!drawn) std::cout << ".";
  }
  std::cout << "|\\n";
  }
  std::cout << "+";
  for(int i=0; i<this->width; i++) std::cout << "-";
  std::cout << "+\\n";
  std::cout << "WASD: Mover | P: Pausa | Q: Salir\\n";
}
bool Game::is_alive(Player p) {
  return (!(p.hp <= 0) && this->state == 1);
}
int Game::math_demo() {
  int a = 1;
  int b = 2;
  return a + b;
}
void Game::input_update(Player p) {
  
  #ifdef _WIN32
  if (_kbhit()) {
  char input = _getch();
  if (this->state == 0) { // Menu
  if (input == 13) this->state = 1; // Enter
  if (input == 'q') this->running = false;
  } else if (this->state == 1) { // Playing
  float dx = 0; float dy = 0;
  if (input == 'w') dy = -1;
  else if (input == 's') dy = 1;
  else if (input == 'a') dx = -1;
  else if (input == 'd') dx = 1;
  else if (input == 'p') this->state = 3;
  else if (input == 'q') this->running = false;
  
  // Predicción y colisión con muros
  float new_x = p.x + dx;
  float new_y = p.y + dy;
  if (new_x >= 0 && new_x < this->width) p.x = new_x;
  if (new_y >= 0 && new_y < this->height) p.y = new_y;
  } else if (this->state == 2) { // Game Over
  if (input == 'q') this->running = false;
  if (input == 13) { // Reset
  this->state = 1;
  p.hp = 100; p.score = 0; p.x = 1; p.y = 1;
  }
  } else if (this->state == 3) { // Pause
  if (input == 'p') this->state = 1;
  }
  }
  #else
  // Fallback simple para Linux/Mac (requiere Enter)
  char input;
  std::cin >> input;
  // ... lógica simplificada ...
  #endif
}
void Game::run_loop() {
  
  Checklist list;
  list.print_status();
  std::cout << "Presione Enter para iniciar el juego...";
  
  Player p;
  Enemy e(15.0, 5.0);
  
  while(this->running) {
  if (this->state == 0) { // Menu
  // Esperando start (handled in input)
  #ifdef _WIN32
  if (_kbhit()) {
  char c = _getch();
  if (c == 13) this->state = 1;
  }
  #else
  std::cin.get(); this->state = 1;
  #endif
  }
  else if (this->state == 1 || this->state == 3) { // Playing or Pause
  this->draw_map(p, e);
  this->input_update(p);
  if (this->state == 1) {
  this->check_collisions(p, e);
  if (p.hp <= 0) this->state = 2;
  }
  // Frame delay simulation
  #ifdef _WIN32
  Sleep(50);
  #endif
  }
  else if (this->state == 2) { // Game Over
  this->clear_screen();
  std::cout << "=== GAME OVER ===\\n";
  std::cout << "Score Final: " << p.score << "\\n";
  std::cout << "Presiona Enter para reiniciar o Q para salir.\\n";
  this->input_update(p);
  }
  }
}
