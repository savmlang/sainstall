#include <iostream>
#include <span>

void main(int argc, char *argv[])
{
  std::span<char *> args(argv, argc);
}