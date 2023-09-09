int main() {
  int x = 5;
  print_digit(x);
  return 0;
}

void print_digit(int digit) {
  asm! {
    "add R1, R0, #41",
    "mov R0, #1",
    "mov R1, R0",
    "mov R2, #1",
    "mov R16, #4",
    "svc #0x80"
  };
  return;
}