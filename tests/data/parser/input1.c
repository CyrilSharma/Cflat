void updateValue(int* ptr, int newValue) {
  *ptr = newValue;
}

int main() {
  int num = 10;
  int* ptr = &num;
  updateValue(ptr, 20);
  return 0;
}