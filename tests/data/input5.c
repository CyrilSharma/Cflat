int main() {
  int x = 0;
  int y = 3;
  int *z = &x;
  z = &y;
  *z = 0;
  int *w = &x;
  *w = 3;
  return 0;
}