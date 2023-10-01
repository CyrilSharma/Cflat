int main() {
  int x = 1;
  x += 7;
  x = x % 4;
  int y = 5;
  while (y >= 0) {
    print_digit(x);
    y -= 1;
  }
  return 0;
}

void print_digit(int digit) {
  asm! {
    "add R3, R0, #48",
    "mov R0, #1",
    "mov R1, SP",
    "sub SP, SP, #-16",
    "str R3, [R1]",
    "mov R2, #1",
    "mov R16, #4",
    "svc #0x80"
  };
  return;
}