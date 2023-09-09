int main() {
  int x = 1000;
  print_number(x);
  return 0;
}

void print_number(int x) {
  if (x == 0) return;
  print_number(x / 10);
  print_digit(x % 10);
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