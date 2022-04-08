#include <iostream>

class Base {
public:
    /* se non mettessi `virtual`, sarebbe chiamato il metodoto m() di questa classe Base anche se lo chiamassimo da una
     * classe derivata!
     * -> C++, a differenza del Java vuole che in una classe i metodi che si vuole siano "overridable" debbano essere
     * preceduti dalla parola chiave virtual (gli oggetti occuperanno leggermente di più in memoria) */
    virtual int m() {
        return 1;
    }
};

/* prima classe derivata */
class Der1: public Base {
public:
    virtual int m() {  // virtual aggiunto solo per coding rule. qui potrebbe essere omesso
        return 2;
    }
};

/* seconda classe derivata */
class Der2: public Base {  // virtual aggiunto solo per coding rule. qui potrebbe essere omesso
public:
    virtual int m() {
        return 3;
    }
};

void print(Base *ptr) {
    std::cout << "Print: " << ptr->m() << std::endl;
}

int main() {
    Base b;
    Der1 d1;
    Der2 d2;

    std::cout << "Base: " << b.m() << std::endl;
    std::cout << "Der1: " << d1.m() << std::endl;
    std::cout << "Der2: " << d2.m() << std::endl;

    print(&b);
    /* a print li passo come puntatori perché le due sottoclassi potrebbero essere più o meno grosse */
    print(&d1);
    print(&d2);

    return 0;
}
