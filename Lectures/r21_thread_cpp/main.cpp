#include <iostream>
#include <thread>

int a = 0;  // non possibile in safe Rust
int count = 0;

/* function based on an int global variable */
void run() {
    while (a >= 0) {
        int before = a;
        a++;
        int after = a;
        if (after - before != 1) {
            std::cout << before << " -> " << after
                      << "(" << after - before << ")\n";
            count++;  // this will cause even more faults
        }
    }
}

// we build two threads and wait for their end
int main() {
    std::thread t1(run);
    std::thread t2(run);

    t1.join();
    t2.join();
    /* ouch... */
    std::cout << count << " faults\n";

    return 0;
}
