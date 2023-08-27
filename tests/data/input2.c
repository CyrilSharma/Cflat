// Function to perform various mutations on x
void performMutations(int *x) {
  // Mutation 1: Increment by 1
  *x = *x + 1;

  // Mutation 2: Multiply by 2
  *x = *x * 2;

  // Mutation 3: Subtract 3
  *x = *x - 3;
  return;
}

int main() {
  int x = 5;
  performMutations(&x);
  x += 2;
  int y = x;
  y += y;
  x -= y;
  return 0;
}