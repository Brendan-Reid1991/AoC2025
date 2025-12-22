#pragma once
#include <filesystem>
#include <fstream>
#include <iostream>
#include <string>

#include <vector>

using StringInput = std::vector<std::string>;

/// @brief Return the project root directory,
/// assuming puzzle inputs are kept inside root/puzzle_input/
/// and this function inside root/cpp/include/reader.h
/// @return std::string
inline std::string relative_pathway() {
  std::filesystem::path header_path = __FILE__;
  return header_path.parent_path().parent_path().parent_path().string() +
         "/puzzle_input/";
}

inline std::string Pathway(uint day) {
  return relative_pathway() + "day" + std::to_string(day) + ".txt";
}

inline StringInput read_puzzle_input(uint day) {
  StringInput puzzle_input;
  std::string path = Pathway(day);
  std::ifstream file(path);
  if (!file.is_open()) {
    std::cerr << "Failed to open file: " << path << std::endl;
    return puzzle_input;
  }
  std::string line;
  while (std::getline(file, line)) {
    puzzle_input.push_back(line);
  }
  return puzzle_input;
}

template <typename T> using PuzzleInput = std::vector<T>;