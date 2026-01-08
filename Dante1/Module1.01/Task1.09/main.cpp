#include <iostream>
#include <iomanip>

int main(){
  int a=2;
  int b=8;
  std::cout << std::fixed << std::setprecision(2);
  std::cout << a << "\n" << b << "\n" << a+b << "\n" << a-b << "\n" << a*b << "\n" << (float)a/b << std::endl;
}
