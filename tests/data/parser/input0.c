int add(int a, int b) {
    return a + b;
}

int subtract(int a, int b) {
    return a - b;
}

int multiply(int a, int b) {
    return a * b;
}

int divide(int a, int b) {
    return a / b;
}

float floatAdd(float a, float b) {
    return a + b;
}

float floatSubtract(float a, float b) {
    return a - b;
}

float floatMultiply(float a, float b) {
    return a * b;
}

float floatDivide(float a, float b) {
    return a / b;
}

int main() {
    int x = 5;
    int y = 3;
    int result = add(x, y);
    float a = 2.5;
    float b = 1.3;
    float floatResult = floatMultiply(a, b);
    
    // Perform further operations as needed
    
    return 0;
}