int main() {
  int x = 5;
  asm! {
    "mov R1, R0",
    "mov R0, R1",
    "mov R1, R0"
  };
  return 0;
}