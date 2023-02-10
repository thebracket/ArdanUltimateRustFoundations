#include <iostream>
#include <thread>

bool is_prime(uint32_t n) {
    for (uint32_t i = 2; i <= n / 2; ++i) {
        if (n % i == 0) return false;
    }
    return true;
}

int main() {
    const auto max = 200000;
    uint32_t count = 0;
    std::thread t1([&count, max]() {
        for (auto i = 2; i < max / 2; i++) {
            if (is_prime(i)) ++count;
        }
        });
    std::thread t2([&count, max]() {
        for (auto i = max / 2; i < max; i++) {
            if (is_prime(i)) ++count;
        }
        });
    t1.join();
    t2.join();
    std::cout << "Found " << count << " prime numbers.\n";
}