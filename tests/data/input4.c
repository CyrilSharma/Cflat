float complexFunc1(float x) {
  if (x <= 0) {
    return x;
  } else {
    return complexFunc1(x - 1) + x;
  }
}

void complexFunc2(int n) {
  for (int i = 1; i <= n; i += 1) {
    if (i / 2 == 0) {
      i += 1;
    } else {
      i -= 2;
    }
    if (i == n / 2) {
        break;
    }
  }
  return;
}

int factorial(int num) {
  if (num == 0 || num == 1) {
    return 1;
  } else {
    return num * factorial(num - 1);
  }
}

int fibonacci(int num) {
  if (num <= 1) {
    return num;
  } else {
    return fibonacci(num - 1) + fibonacci(num - 2);
  }
}

int main() {
  int x = 5;
  float result = complexFunc1(1.8);
  for (int i = 0; i < x; i += 1) {
    i -= 2;
  }
  complexFunc2(9);
  return 0;
}