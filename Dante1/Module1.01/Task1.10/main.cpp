#include <iostream>
#include <iomanip>

int main(){
  float a=6.4;
  float b=3.9;
  std::cout << std::fixed << std::setprecision(2);
  std::cout << a << "\n" << b << "\n" << a+b << "\n" << a-b << "\n" << a*b << "\n" << (float)a/b << std::endl;
}
