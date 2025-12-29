#include "resourcemanager.hpp"
#include "pch.hpp"
#include <string>
#include <vector>
#include <iostream>
namespace Juego {
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
}
