/**
 * Esercizio per vedere il paradigma RAII in C++
 */

#include <iostream>

class Test {
public:
    /* costruttore */
    Test() {
        std::cout<<"Un ogetto di tipo test è stato costruito all'indirizzo "<<this<<std::endl;
    }
    /* distruttore */
    ~Test() {
        std::cout<<"Un ogetto di tipo test all'indirizzo "<<this<<" è stato distrutto"<<std::endl;
    }
};

int f() {
    Test t1;
    for (int i = 0; i < 2; i++) {
        Test t;
        if (i == 1) throw std::exception();
        std::cout<<"---"<<std::endl;
    }
    return 0;
}

int main() {
    try {
        f();
    }
    catch (...) {
        std::cout<<"\nPulizia dello stack e chiamata al distruttore anche se il programma muore malamente."<<std::endl;
        std::cout<<"Questa è la base del paradigma RAII."<<std::endl;

    }
}
