#include <iostream>
#include <iomanip>

int main(){
    float fPI = 3.14159265359;
    double dPI = 3.14159265359;
    std::cout << std::fixed << std::setprecision(11);
    std::cout << fPI << std::endl; 
    std::cout << dPI <<	std::endl;
}