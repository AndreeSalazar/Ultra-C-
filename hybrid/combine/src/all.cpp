#include <iostream>
#include <string>
#include <vector>
#include <memory>
#include <algorithm>
#include <map>
#include <list>
#include <optional>
#include <thread>
#include <mutex>
#include <future>
#include <atomic>
#include <filesystem>
#include <fstream>
#include <numeric>
#include <cmath>
#include <cstdio>
#include "pch.hpp"
#include <functional>

// Forward Declarations
namespace Juego { class EventBus; }
namespace Juego { class Rect; }
namespace Juego { class Sound; }
namespace Juego { class ResourceManager; }
namespace Juego { class Config; }
namespace Juego { class ConfigSchema; }
namespace Juego { class Strings; }
namespace Juego { class Entity; }
namespace Juego { class Player; }
namespace Juego { class Principal; }
namespace Juego { class Hola; }
namespace Utils { class Version; }

// Class Definitions
namespace Juego {
#ifndef UCPP_API
#  if defined(_WIN32) && defined(UCPP_DLL)
#    ifdef UCPP_BUILD
#      define UCPP_API __declspec(dllexport)
#    else
#      define UCPP_API __declspec(dllimport)
#    endif
#  elif defined(__GNUC__)
#    define UCPP_API __attribute__((visibility("default")))
#  else
#    define UCPP_API
#  endif
#endif
#ifndef UCPP_NOEXCEPT
#  define UCPP_NOEXCEPT noexcept
#endif
class UCPP_API EventBus {
public:
  std::map<std::string, std::string> audio_cat;
  std::map<std::string, int> audio_prio;
  EventBus();
  EventBus(std::map<std::string, std::string> audio_cat, std::map<std::string, int> audio_prio);
  void subscribe(const std::string& event, const std::string& name);
  void set_audio(const std::string& event, const std::string& cat, int prio);
  void emit(const std::string& event, const std::string& payload);
};

#ifndef UCPP_API
#  if defined(_WIN32) && defined(UCPP_DLL)
#    ifdef UCPP_BUILD
#      define UCPP_API __declspec(dllexport)
#    else
#      define UCPP_API __declspec(dllimport)
#    endif
#  elif defined(__GNUC__)
#    define UCPP_API __attribute__((visibility("default")))
#  else
#    define UCPP_API
#  endif
#endif
#ifndef UCPP_NOEXCEPT
#  define UCPP_NOEXCEPT noexcept
#endif
class rect;
class UCPP_API Rect {
public:
  float x;
  float y;
  float w;
  float h;
  Rect();
  Rect(float x, float y, float w, float h);
  bool collides(const Rect& other) const UCPP_NOEXCEPT;
};

#ifndef UCPP_API
#  if defined(_WIN32) && defined(UCPP_DLL)
#    ifdef UCPP_BUILD
#      define UCPP_API __declspec(dllexport)
#    else
#      define UCPP_API __declspec(dllimport)
#    endif
#  elif defined(__GNUC__)
#    define UCPP_API __attribute__((visibility("default")))
#  else
#    define UCPP_API
#  endif
#endif
#ifndef UCPP_NOEXCEPT
#  define UCPP_NOEXCEPT noexcept
#endif
class UCPP_API Sound {
public:
  static void play(const std::string& category, int priority, const std::string& message);
};

#ifndef UCPP_API
#  if defined(_WIN32) && defined(UCPP_DLL)
#    ifdef UCPP_BUILD
#      define UCPP_API __declspec(dllexport)
#    else
#      define UCPP_API __declspec(dllimport)
#    endif
#  elif defined(__GNUC__)
#    define UCPP_API __attribute__((visibility("default")))
#  else
#    define UCPP_API
#  endif
#endif
#ifndef UCPP_NOEXCEPT
#  define UCPP_NOEXCEPT noexcept
#endif
class UCPP_API ResourceManager {
public:
  std::map<std::string, std::string> resources;
  std::map<std::string, int> refs;
  ResourceManager();
  ResourceManager(std::map<std::string, std::string> resources, std::map<std::string, int> refs);
  void load(const std::string& name, const std::string& content);
  std::string get(const std::string& name) const UCPP_NOEXCEPT;
  void release(const std::string& name) UCPP_NOEXCEPT;
};

#ifndef UCPP_API
#  if defined(_WIN32) && defined(UCPP_DLL)
#    ifdef UCPP_BUILD
#      define UCPP_API __declspec(dllexport)
#    else
#      define UCPP_API __declspec(dllimport)
#    endif
#  elif defined(__GNUC__)
#    define UCPP_API __attribute__((visibility("default")))
#  else
#    define UCPP_API
#  endif
#endif
#ifndef UCPP_NOEXCEPT
#  define UCPP_NOEXCEPT noexcept
#endif
class UCPP_API Config {
public:
  int width;
  int height;
  int start_x;
  int start_y;
  std::vector<Rect> obstacles;
  std::string profile;
  std::string sprite_player;
  std::string sprite_obstacle;
  std::string audio_map;
  std::string lang;
  Config();
  Config(int width, int height, int start_x, int start_y, std::vector<Rect> obstacles, std::string profile, std::string sprite_player, std::string sprite_obstacle, std::string audio_map, std::string lang);
  void add_obstacle(int x, int y);
  void set_size(int w, int h);
  void set_start(int x, int y);
  static Config load(const std::string& path);
};

#ifndef UCPP_API
#  if defined(_WIN32) && defined(UCPP_DLL)
#    ifdef UCPP_BUILD
#      define UCPP_API __declspec(dllexport)
#    else
#      define UCPP_API __declspec(dllimport)
#    endif
#  elif defined(__GNUC__)
#    define UCPP_API __attribute__((visibility("default")))
#  else
#    define UCPP_API
#  endif
#endif
#ifndef UCPP_NOEXCEPT
#  define UCPP_NOEXCEPT noexcept
#endif
class config;
class UCPP_API ConfigSchema {
public:
  bool validate(const Config& cfg);
};

#ifndef UCPP_API
#  if defined(_WIN32) && defined(UCPP_DLL)
#    ifdef UCPP_BUILD
#      define UCPP_API __declspec(dllexport)
#    else
#      define UCPP_API __declspec(dllimport)
#    endif
#  elif defined(__GNUC__)
#    define UCPP_API __attribute__((visibility("default")))
#  else
#    define UCPP_API
#  endif
#endif
#ifndef UCPP_NOEXCEPT
#  define UCPP_NOEXCEPT noexcept
#endif
class UCPP_API Strings {
public:
  std::map<std::string, std::string> table;
  Strings();
  Strings(std::map<std::string, std::string> table);
  void load(const std::string& lang);
  std::string get(const std::string& key) const UCPP_NOEXCEPT;
};

#ifndef UCPP_API
#  if defined(_WIN32) && defined(UCPP_DLL)
#    ifdef UCPP_BUILD
#      define UCPP_API __declspec(dllexport)
#    else
#      define UCPP_API __declspec(dllimport)
#    endif
#  elif defined(__GNUC__)
#    define UCPP_API __attribute__((visibility("default")))
#  else
#    define UCPP_API
#  endif
#endif
#ifndef UCPP_NOEXCEPT
#  define UCPP_NOEXCEPT noexcept
#endif
class UCPP_API Entity {
public:
  float x;
  float y;
  Entity();
  Entity(float x, float y);
  void move(float dx, float dy);
};

#ifndef UCPP_API
#  if defined(_WIN32) && defined(UCPP_DLL)
#    ifdef UCPP_BUILD
#      define UCPP_API __declspec(dllexport)
#    else
#      define UCPP_API __declspec(dllimport)
#    endif
#  elif defined(__GNUC__)
#    define UCPP_API __attribute__((visibility("default")))
#  else
#    define UCPP_API
#  endif
#endif
#ifndef UCPP_NOEXCEPT
#  define UCPP_NOEXCEPT noexcept
#endif
class UCPP_API Player : public Entity {
public:
  std::string name;
  Player();
  Player(std::string name);
  void render() UCPP_NOEXCEPT;
  Rect get_rect() const UCPP_NOEXCEPT;
};

#ifndef UCPP_API
#  if defined(_WIN32) && defined(UCPP_DLL)
#    ifdef UCPP_BUILD
#      define UCPP_API __declspec(dllexport)
#    else
#      define UCPP_API __declspec(dllimport)
#    endif
#  elif defined(__GNUC__)
#    define UCPP_API __attribute__((visibility("default")))
#  else
#    define UCPP_API
#  endif
#endif
#ifndef UCPP_NOEXCEPT
#  define UCPP_NOEXCEPT noexcept
#endif
class UCPP_API Principal {
public:
  bool running;
  int score;
  int level;
  bool paused;
  int target_fps;
  int width;
  int height;
  int high_score;
  std::string profile;
  Player player;
  std::vector<Rect> obstacles;
  EventBus bus;
  ResourceManager rm;
  std::string ascii_player;
  std::string ascii_obstacle;
  Strings strings;
  Principal();
  Principal(bool running, int score, int level, bool paused, int target_fps, int width, int height, int high_score, std::string profile, Player player, std::vector<Rect> obstacles, EventBus bus, ResourceManager rm, std::string ascii_player, std::string ascii_obstacle, Strings strings);
  void update(float dt);
  void draw();
  void run_loop();
  void hola_upp();
  void start();
  void loop();
  void run();
};

#ifndef UCPP_API
#  if defined(_WIN32) && defined(UCPP_DLL)
#    ifdef UCPP_BUILD
#      define UCPP_API __declspec(dllexport)
#    else
#      define UCPP_API __declspec(dllimport)
#    endif
#  elif defined(__GNUC__)
#    define UCPP_API __attribute__((visibility("default")))
#  else
#    define UCPP_API
#  endif
#endif
#ifndef UCPP_NOEXCEPT
#  define UCPP_NOEXCEPT noexcept
#endif
class UCPP_API Hola {
public:
  std::string name;
  Hola();
  Hola(std::string name);
  std::string greet();
};

}
namespace Utils {
#ifndef UCPP_API
#  if defined(_WIN32) && defined(UCPP_DLL)
#    ifdef UCPP_BUILD
#      define UCPP_API __declspec(dllexport)
#    else
#      define UCPP_API __declspec(dllimport)
#    endif
#  elif defined(__GNUC__)
#    define UCPP_API __attribute__((visibility("default")))
#  else
#    define UCPP_API
#  endif
#endif
#ifndef UCPP_NOEXCEPT
#  define UCPP_NOEXCEPT noexcept
#endif
class UCPP_API Version {
public:
  static std::string current();
};

}

// Class Implementations
namespace Juego {
EventBus::EventBus() : audio_cat(std::map<std::string, std::string>()), audio_prio(std::map<std::string, int>()) {}
EventBus::EventBus(std::map<std::string, std::string> audio_cat, std::map<std::string, int> audio_prio) : audio_cat(audio_cat), audio_prio(audio_prio) {
}
void EventBus::subscribe(const std::string& event, const std::string& name) {
  std::cout << std::string("Event: ") + event + std::string(" payload=") + name << std::endl;
}
void EventBus::set_audio(const std::string& event, const std::string& cat, int prio) {
  audio_cat[event] = cat; audio_prio[event] = prio;
}
void EventBus::emit(const std::string& event, const std::string& payload) {
  std::string cat = "events";
  int pr = 5;
  auto itc = audio_cat.find(event);
  if (itc != audio_cat.end()) { cat = itc->second; }
  auto itp = audio_prio.find(event);
  if (itp != audio_prio.end()) { pr = itp->second; }
  Sound::play(cat, pr, payload);
}
Rect::Rect() : x(0.0f), y(0.0f), w(0.0f), h(0.0f) {}
Rect::Rect(float x, float y, float w, float h) : x(x), y(y), w(w), h(h) {
}
bool Rect::collides(const Rect& other) const UCPP_NOEXCEPT {
  return (x < other.x + other.w) && (x + w > other.x) && (y < other.y + other.h) && (y + h > other.y);
}
void Sound::play(const std::string& category, int priority, const std::string& message) {
  std::cout << "[SND][" << category << "][prio=" << priority << "] " << message << std::endl;
}
ResourceManager::ResourceManager() : resources(std::map<std::string, std::string>()), refs(std::map<std::string, int>()) {}
ResourceManager::ResourceManager(std::map<std::string, std::string> resources, std::map<std::string, int> refs) : resources(resources), refs(refs) {
}
void ResourceManager::load(const std::string& name, const std::string& content) {
  if (refs.find(name) == refs.end()) { refs[name] = 1; resources[name] = content; }
  else { refs[name] += 1; }
}
std::string ResourceManager::get(const std::string& name) const UCPP_NOEXCEPT {
  auto it = resources.find(name);
  if (it != resources.end()) { return it->second; }
  return std::string("");
}
void ResourceManager::release(const std::string& name) UCPP_NOEXCEPT {
  auto it = refs.find(name);
  if (it != refs.end()) {
  if (--(it->second) <= 0) { refs.erase(it); resources.erase(name); }
  }
}
Config::Config() : width(0), height(0), start_x(0), start_y(0), obstacles(std::vector<Rect>()), profile(std::string()), sprite_player(std::string()), sprite_obstacle(std::string()), audio_map(std::string()), lang(std::string()) {}
Config::Config(int width, int height, int start_x, int start_y, std::vector<Rect> obstacles, std::string profile, std::string sprite_player, std::string sprite_obstacle, std::string audio_map, std::string lang) : width(width), height(height), start_x(start_x), start_y(start_y), obstacles(obstacles), profile(profile), sprite_player(sprite_player), sprite_obstacle(sprite_obstacle), audio_map(audio_map), lang(lang) {
}
void Config::add_obstacle(int x, int y) {
  Rect r; r.x = (float)x; r.y = (float)y; r.w = 1.0f; r.h = 1.0f; obstacles.push_back(r);
}
void Config::set_size(int w, int h) {
  width = w; height = h;
}
void Config::set_start(int x, int y) {
  start_x = x; start_y = y;
}
Config Config::load(const std::string& path) {
  Config cfg;
  cfg.width = 40; cfg.height = 12; cfg.start_x = 2; cfg.start_y = 2; cfg.profile = std::string(""); cfg.sprite_player = std::string(""); cfg.sprite_obstacle = std::string(""); cfg.audio_map = std::string(""); cfg.lang = std::string("es");
  std::ifstream f(path);
  if (f) {
  auto trim = [](std::string s)->std::string {
  size_t a=0,b=s.size();
  while (a<b && (s[a]==' '||s[a]=='\t')) a++;
  while (b>a && (s[b-1]==' '||s[b-1]=='\t'||s[b-1]=='\r'||s[b-1]=='\n')) b--;
  return s.substr(a,b-a);
  };
  std::string line;
  while (std::getline(f, line)) {
  auto hash = line.find('#'); if (hash != std::string::npos) line = line.substr(0, hash);
  line = trim(line);
  if (line.empty()) continue;
  if (line.find("width") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.width = std::stoi(trim(line.substr(eq+1))); }
  } else if (line.find("height") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.height = std::stoi(trim(line.substr(eq+1))); }
  } else if (line.find("player_x") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.start_x = std::stoi(trim(line.substr(eq+1))); }
  } else if (line.find("player_y") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.start_y = std::stoi(trim(line.substr(eq+1))); }
  } else if (line.find("sprite_player") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.sprite_player = trim(line.substr(eq+1)); }
  } else if (line.find("sprite_obstacle") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.sprite_obstacle = trim(line.substr(eq+1)); }
  } else if (line.find("profile") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.profile = trim(line.substr(eq+1)); }
  } else if (line.find("audio_map") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.audio_map = trim(line.substr(eq+1)); }
  } else if (line.find("lang") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) { cfg.lang = trim(line.substr(eq+1)); }
  } else if (line.find("obstacles") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) {
  auto s = line.substr(eq+1);
  std::vector<int> nums; nums.reserve(64);
  std::string cur;
  for (char ch : s) {
  if ((ch>='0'&&ch<='9') || ch=='-') { cur.push_back(ch); }
  else { if (!cur.empty()) { nums.push_back(std::stoi(cur)); cur.clear(); } }
  }
  if (!cur.empty()) { nums.push_back(std::stoi(cur)); cur.clear(); }
  for (size_t i=0; i+1<nums.size(); i+=2) {
  Rect r; r.x = (float)nums[i]; r.y = (float)nums[i+1]; r.w=1.0f; r.h=1.0f; cfg.obstacles.push_back(r);
  }
  }
  }
  }
  }
  if (cfg.width <= 0 || cfg.height <= 0) { cfg.width = 40; cfg.height = 12; Sound::play(std::string("config"), 1, std::string("invalid width/height; defaults applied")); }
  if (!cfg.profile.empty()) {
  std::string pfile = std::string("config.") + cfg.profile + std::string(".toml");
  std::ifstream pf(pfile);
  if (pf) {
  auto trim = [](std::string s)->std::string {
  size_t a=0,b=s.size();
  while (a<b && (s[a]==' '||s[a]=='\t')) a++;
  while (b>a && (s[b-1]==' '||s[b-1]=='\t'||s[b-1]=='\r'||s[b-1]=='\n')) b--;
  return s.substr(a,b-a);
  };
  std::string line;
  while (std::getline(pf, line)) {
  auto hash = line.find('#'); if (hash != std::string::npos) line = line.substr(0, hash);
  line = trim(line);
  if (line.empty()) continue;
  if (line.find("width") == 0) { auto eq = line.find('='); if (eq!=std::string::npos) { cfg.width = std::stoi(trim(line.substr(eq+1))); } }
  else if (line.find("height") == 0) { auto eq = line.find('='); if (eq!=std::string::npos) { cfg.height = std::stoi(trim(line.substr(eq+1))); } }
  else if (line.find("player_x") == 0) { auto eq = line.find('='); if (eq!=std::string::npos) { cfg.start_x = std::stoi(trim(line.substr(eq+1))); } }
  else if (line.find("player_y") == 0) { auto eq = line.find('='); if (eq!=std::string::npos) { cfg.start_y = std::stoi(trim(line.substr(eq+1))); } }
  else if (line.find("sprite_player") == 0) { auto eq = line.find('='); if (eq!=std::string::npos) { cfg.sprite_player = trim(line.substr(eq+1)); } }
  else if (line.find("sprite_obstacle") == 0) { auto eq = line.find('='); if (eq!=std::string::npos) { cfg.sprite_obstacle = trim(line.substr(eq+1)); } }
  else if (line.find("audio_map") == 0) { auto eq = line.find('='); if (eq!=std::string::npos) { cfg.audio_map = trim(line.substr(eq+1)); } }
  else if (line.find("lang") == 0) { auto eq = line.find('='); if (eq!=std::string::npos) { cfg.lang = trim(line.substr(eq+1)); } }
  else if (line.find("obstacles") == 0) {
  auto eq = line.find('='); if (eq!=std::string::npos) {
  auto s = line.substr(eq+1);
  std::vector<int> nums; nums.reserve(64);
  std::string cur;
  for (char ch : s) {
  if ((ch>='0'&&ch<='9') || ch=='-') { cur.push_back(ch); }
  else { if (!cur.empty()) { nums.push_back(std::stoi(cur)); cur.clear(); } }
  }
  if (!cur.empty()) { nums.push_back(std::stoi(cur)); cur.clear(); }
  cfg.obstacles.clear();
  for (size_t i=0; i+1<nums.size(); i+=2) {
  Rect r; r.x = (float)nums[i]; r.y = (float)nums[i+1]; r.w=1.0f; r.h=1.0f; cfg.obstacles.push_back(r);
  }
  }
  }
  }
  }
  }
  return cfg;
}
bool ConfigSchema::validate(const Config& cfg) {
  if (cfg.width < 10 || cfg.width > 100) { std::cerr << "config width out of range" << std::endl; return false; }
  if (cfg.height < 5 || cfg.height > 60) { std::cerr << "config height out of range" << std::endl; return false; }
  if (cfg.start_x < 0 || cfg.start_x >= cfg.width || cfg.start_y < 0 || cfg.start_y >= cfg.height) { std::cerr << "player start out of bounds" << std::endl; return false; }
  return true;
}
Strings::Strings() : table(std::map<std::string, std::string>()) {}
Strings::Strings(std::map<std::string, std::string> table) : table(table) {
}
void Strings::load(const std::string& lang) {
  table.clear();
  std::string fname = std::string("strings_") + lang + std::string(".txt");
  std::ifstream f(fname);
  auto trim = [](std::string s)->std::string {
  size_t a=0,b=s.size();
  while (a<b && (s[a]==' '||s[a]=='\t')) a++;
  while (b>a && (s[b-1]==' '||s[b-1]=='\t'||s[b-1]=='\r'||s[b-1]=='\n')) b--;
  return s.substr(a,b-a);
  };
  if (f) {
  std::string line;
  while (std::getline(f, line)) {
  auto hash = line.find('#'); if (hash != std::string::npos) line = line.substr(0, hash);
  auto eq = line.find('=');
  if (eq != std::string::npos) {
  std::string k = trim(line.substr(0, eq));
  std::string v = trim(line.substr(eq+1));
  if (!k.empty()) table[k] = v;
  }
  }
  }
  if (table.find("version_label") == table.end()) table["version_label"] = "Versión actual:";
  if (table.find("start_label") == table.end()) table["start_label"] = "--- Start ---";
  if (table.find("end_label") == table.end()) table["end_label"] = "--- Fin Ejecución ---";
}
std::string Strings::get(const std::string& key) const UCPP_NOEXCEPT {
  auto it = table.find(key);
  if (it != table.end()) return it->second;
  return key;
}
Entity::Entity() : x(0.0f), y(0.0f) {}
Entity::Entity(float x, float y) : x(x), y(y) {
}
void Entity::move(float dx, float dy) {
  x += dx; y += dy;
}
Player::Player() : name(std::string()) {}
Player::Player(std::string name) : name(name) {
}
void Player::render() UCPP_NOEXCEPT {
  std::cout << std::string("Player: ") + name << std::endl;
}
Rect Player::get_rect() const UCPP_NOEXCEPT {
  Rect r; r.x = x; r.y = y; r.w = 1.0f; r.h = 1.0f; return r;
}
#ifdef _WIN32
#include <conio.h>
#include <windows.h>
#endif
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
Hola::Hola() : name(std::string()) {}
Hola::Hola(std::string name) : name(name) {
}
std::string Hola::greet() {
  return std::string("Hola ") + name;
}
}
namespace Utils {
std::string Version::current() {
  return "1.0.0";
}
}


#ifdef _WIN32
#include <windows.h>
#endif
int main() {
  #ifdef _WIN32
    // SetConsoleOutputCP(65001); // disabled for broader toolchain compatibility
  #endif
  try {
    Juego::Principal app;
    app.run();
  } catch (const std::exception& e) {
    std::cerr << e.what() << std::endl;
    return 1;
  }
  return 0;
}
