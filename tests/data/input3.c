int add(int a, int b) {
  return a + b;
}

float floatDivide(float a, float b) {
  if (b == 0) {
    return -1.0; // Early return
  }
  return a / b;
}

int main() {
  int x = 5;
  int y = 3;
  int result = add(x, y);
  float a = 2.5;
  float b = 0.0; // Try changing b to 0 to trigger division by zero error
  float floatResult = floatDivide(a, b);
  
  if (floatResult >= 0) {
    b += 2.0;
  }
  
  return 0;
}