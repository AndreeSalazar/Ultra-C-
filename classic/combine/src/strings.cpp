#include "strings.hpp"
#include <string>
#include <vector>
#include <iostream>
namespace Juego {
Strings::Strings() : table(std::map<std::string, std::string>()) {}
Strings::Strings(std::map<std::string, std::string> table) : table(table) {
}
void Strings::load(std::string lang) {
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
std::string Strings::get(std::string key) const {
  auto it = table.find(key);
  if (it != table.end()) return it->second;
  return key;
}
}
