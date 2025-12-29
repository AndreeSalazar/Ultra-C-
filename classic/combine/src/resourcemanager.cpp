#include "resourcemanager.hpp"
#include <string>
#include <vector>
#include <iostream>
namespace Juego {
ResourceManager::ResourceManager() : resources(std::map<std::string, std::string>()), refs(std::map<std::string, int>()) {}
ResourceManager::ResourceManager(std::map<std::string, std::string> resources, std::map<std::string, int> refs) : resources(resources), refs(refs) {
}
void ResourceManager::load(std::string name, std::string content) {
  if (refs.find(name) == refs.end()) { refs[name] = 1; resources[name] = content; }
  else { refs[name] += 1; }
}
std::string ResourceManager::get(std::string name) const {
  auto it = resources.find(name);
  if (it != resources.end()) { return it->second; }
  return std::string("");
}
void ResourceManager::release(std::string name) {
  auto it = refs.find(name);
  if (it != refs.end()) {
  if (--(it->second) <= 0) { refs.erase(it); resources.erase(name); }
  }
}
}
