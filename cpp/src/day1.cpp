#include "parsers.h"
#include <iostream>

int main() {
  PuzzleInput<Day1Line> inputs = parse_day_1();
  int current_position = 50;
  int count_zeros = 0;
  for (const auto &[direction, value] : inputs) {
    if (direction == 'R') {
      current_position -= value;
    } else {
      current_position += value;
    }
    current_position = (current_position + 100) % 100;
    if (current_position == 0) {
      count_zeros += 1;
    }
  }
  std::cout << count_zeros << std::endl;
  return 0;
}