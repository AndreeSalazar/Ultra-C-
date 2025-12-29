#include "principal.hpp"
#include "pch.hpp"
#include "config.hpp"
#include "sound.hpp"
#include "hola.hpp"
#include "version.hpp"
#include "config.hpp"
#include "configschema.hpp"
#include "config.hpp"
#include "sound.hpp"
#include "entity.hpp"
#include <string>
#include <vector>
#include <iostream>
#ifdef _WIN32
#include <conio.h>
#include <windows.h>
#endif
namespace Juego {
Principal::Principal() : running(false), score(0), level(0), paused(false), target_fps(0), width(0), height(0), high_score(0), profile(std::string()), player(Player()), obstacles(std::vector<Rect>()), bus(EventBus()), rm(ResourceManager()), ascii_player(std::string()), ascii_obstacle(std::string()), strings(Strings()) {}
Principal::Principal(bool running, int score, int level, bool paused, int target_fps, int width, int height, int high_score, std::string profile, Player player, std::vector<Rect> obstacles, EventBus bus, ResourceManager rm, std::string ascii_player, std::string ascii_obstacle, Strings strings) : running(running), score(score), level(level), paused(paused), target_fps(target_fps), width(width), height(height), high_score(high_score), profile(profile), player(player), obstacles(obstacles), bus(bus), rm(rm), ascii_player(ascii_player), ascii_obstacle(ascii_obstacle), strings(strings) {
}
void Principal::update(float dt) {
  if (paused) {
    return;
  }
  (void)dt;
  #ifdef _WIN32
  if (_kbhit()) {
  int c = _getch();
  if (c == 'w') { this->player.move(0.0f, -0.2f); }
  else if (c == 's') { this->player.move(0.0f, 0.2f); }
  else if (c == 'a') { this->player.move(-0.2f, 0.0f); }
  else if (c == 'd') { this->player.move(0.2f, 0.0f); }
  else if (c == 'p') { this->paused = !this->paused; }
  }
  #endif
  Rect pr = this->player.get_rect();
  for (auto &o : this->obstacles) {
  if (pr.collides(o)) {
  this->score += 1;
  if (this->score > this->high_score) { this->high_score = this->score; }
  this->bus.emit(std::string("collision"), std::string("player"));
  break;
  }
  }
}
void Principal::draw() {
  int w = width, h = height;
  if (w <= 0) w = 40;
  if (h <= 0) h = 12;
  std::vector<std::string> rows; rows.assign(h, std::string(w, '.'));
  for (auto &o : obstacles) {
  int ox = (int)o.x, oy = (int)o.y;
  if (ox>=0 && ox<w && oy>=0 && oy<h) rows[oy][ox] = ascii_obstacle.empty()? 'O' : ascii_obstacle[0];
  }
  int px = (int)player.x, py = (int)player.y;
  if (px>=0 && px<w && py>=0 && py<h) rows[py][px] = ascii_player.empty()? 'P' : ascii_player[0];
  std::string block;
  block.reserve(h*(w+1)+64);
  for (int y=0; y<h; ++y) { block += rows[y]; block.push_back('\n'); }
  block += std::string("Score ") + std::to_string(score) + std::string(" Level ") + std::to_string(level) + std::string(" High ") + std::to_string(high_score) + std::string("\n");
  std::cout << block;
}
void Principal::run_loop() {
  static std::time_t last_m = 0;
  for (int i = 0; i < 300; ++i) {
  float dt = 1.0f / this->target_fps;
  this->update(dt);
  {
  std::error_code ec;
  auto p = std::filesystem::path("config.toml");
  if (std::filesystem::exists(p, ec)) {
  auto ft = std::filesystem::last_write_time(p, ec);
  auto sctp = std::chrono::time_point_cast<std::chrono::system_clock::duration>(
  ft - std::filesystem::file_time_type::clock::now() + std::chrono::system_clock::now()
  );
  std::time_t mt = std::chrono::system_clock::to_time_t(sctp);
  if (mt > last_m) {
  last_m = mt;
  Config cfg2 = Config::load(std::string("config.toml"));
  width = cfg2.width; height = cfg2.height; player.x = (float)cfg2.start_x; player.y = (float)cfg2.start_y; obstacles = cfg2.obstacles;
  ConfigSchema sch;
  if (!sch.validate(cfg2)) { Sound::play(std::string("config"), 1, std::string("invalid reload; defaults")); width = 40; height = 12; player.x = 2.0f; player.y = 2.0f; }
  if (!cfg2.sprite_player.empty()) ascii_player = cfg2.sprite_player;
  if (!cfg2.sprite_obstacle.empty()) ascii_obstacle = cfg2.sprite_obstacle;
  if (!cfg2.audio_map.empty()) {
  std::string s = cfg2.audio_map;
  std::string cur;
  for (size_t k=0;k<=s.size();++k) {
  char ch = (k<s.size()? s[k] : ';');
  if (ch!=';') cur.push_back(ch);
  else {
  if (!cur.empty()) {
  std::vector<std::string> parts;
  std::string tmp;
  for (size_t m=0;m<=cur.size();++m) {
  char c = (m<cur.size()? cur[m] : ':');
  if (c!=':') tmp.push_back(c); else { parts.push_back(tmp); tmp.clear(); }
  }
  if (parts.size()>=2) {
  std::string ev = parts[0];
  std::string cat = parts[1];
  int pr = (parts.size()>=3)? std::stoi(parts[2]) : 5;
  bus.set_audio(ev, cat, pr);
  }
  }
  cur.clear();
  }
  }
  }
  Sound::play(std::string("config"), 2, std::string("reload"));
  }
  }
  }
  this->draw();
  #ifdef _WIN32
  Sleep(1000 / this->target_fps);
  #else
  // sin chrono: no-op o sleep simple según plataforma
  #endif
  }
}
void Principal::hola_upp() {
  std::cout << strings.get(std::string("version_label")) << " " << Utils::Version::current() << std::endl;
  auto h = Hola("como están");
  std::cout << h.greet() << std::endl;
}
void Principal::start() {
  std::cout << strings.get(std::string("start_label")) << std::endl;
  running = true; score = 0; level = 1; paused = false; target_fps = 60;
  player.name = std::string("Eddi");
  auto cfg = Config::load("config.toml");
  width = cfg.width; height = cfg.height; player.x = (float)cfg.start_x; player.y = (float)cfg.start_y; this->obstacles = cfg.obstacles; profile = cfg.profile;
  auto sch = ConfigSchema();
  if (!sch.validate(cfg)) { Sound::play(std::string("config"), 1, std::string("invalid; defaults")); width = 40; height = 12; player.x = 2.0f; player.y = 2.0f; }
  rm.load("player", "P");
  rm.load("obstacle", "O");
  ascii_player = rm.get(std::string("player")); if (!cfg.sprite_player.empty()) ascii_player = cfg.sprite_player;
  ascii_obstacle = rm.get(std::string("obstacle")); if (!cfg.sprite_obstacle.empty()) ascii_obstacle = cfg.sprite_obstacle;
  strings.load(cfg.lang);
  if (!cfg.audio_map.empty()) { std::string s = cfg.audio_map; std::string cur; for (size_t k=0;k<=s.size();++k) { char ch = (k<s.size()? s[k] : ';'); if (ch!=';') cur.push_back(ch); else { if (!cur.empty()) { std::vector<std::string> parts; std::string tmp; for (size_t m=0;m<=cur.size();++m) { char c = (m<cur.size()? cur[m] : ':'); if (c!=':') tmp.push_back(c); else { parts.push_back(tmp); tmp.clear(); } } if (parts.size()>=2) { std::string ev = parts[0]; std::string cat = parts[1]; int pr = (parts.size()>=3)? std::stoi(parts[2]) : 5; bus.set_audio(ev, cat, pr); } } cur.clear(); } } }
  Sound::play(std::string("system"), 1, std::string("start")); bus.emit(std::string("start"), std::string(""));
  Rect o; o.x = 1.0f; o.y = 1.0f; o.w = 1.0f; o.h = 1.0f;
  this->obstacles.push_back(o);
  high_score = 0; { std::ifstream hs("highscore.txt"); if (hs) { int v=0; hs>>v; if (v>0) high_score = v; } }
  hola_upp();
  player.render();
}
void Principal::loop() {
  std::cout << "--- Loop Tick ---" << std::endl;
  score += 1;
  level += (score % 3 == 0) ? 1 : 0;
  auto e = Entity();
  e.move(0.5f, 0.25f);
  hola_upp();
}
void Principal::run() {
  std::cout << strings.get(std::string("start_label")) << std::endl;
  start();
  run_loop();
  rm.release(std::string("player")); rm.release(std::string("obstacle"));
  { std::ofstream hs("highscore.txt"); hs << high_score; }
  std::cout << strings.get(std::string("end_label")) << std::endl;
}
}
