int main() {
  int x = 1000;
  print_number(x);
  return 0;
}

void print_number(int x) {
  int mx = 1;
  int tmp = x;
  while (tmp > 0) {
    tmp /= 10;
    mx *= 10;
  }
  while (mx > 0) {
    print_digit(x / mx);
    mx /= 10;
  }
  return;
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