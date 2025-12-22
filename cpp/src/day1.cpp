#include "parsers.h"
#include <iostream>

int main() {
  PuzzleInput<Day1T> inputs = parse_day_1();
  int position = 50;
  int hits_zero = 0;
  int new_position = 0;
  for (const auto &value : inputs) {
    hits_zero += std::abs(value) / 100;
    new_position = ((position + value) % 100 + 100) % 100;
    if (position != 0 &&
        ((value > 0 && new_position < position) ||
         (value < 0 && new_position > position) || new_position == 0)) {
      hits_zero += 1;
    }
    position = new_position;
  }
  std::cout << hits_zero << std::endl;
  return 0;
}