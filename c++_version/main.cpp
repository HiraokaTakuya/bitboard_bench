#include <iostream>
#include <chrono>
#include <cstdint>

enum Square : int32_t {};

struct Bitboard {
    uint64_t value(size_t i) const { return v[i]; }
    uint64_t merge() const { return value(0) | value(1); }
    explicit operator bool() const { return merge() != 0; }
    Square pop_lsb_right_unchecked() {
        auto sq = Square(__builtin_ctzll(value(0)));
        v[0] &= v[0] - 1;
        return sq;
    }
    Square pop_lsb_left_unchecked() {
        auto sq = Square(__builtin_ctzll(value(1)) + 64);
        v[1] &= v[1] - 1;
        return sq;
    }
    Square pop_lsb_unchecked() {
        if (value(0) != 0)
            return pop_lsb_right_unchecked();
        return pop_lsb_left_unchecked();
    }

    uint64_t v[2];
};

const Bitboard ALL = {0x7fffffffffffffff, 0x7fffffffffffffff};

int main() {
    constexpr int64_t NUM_TRIALS = 30000000;
    auto start = std::chrono::system_clock::now();
    uint64_t sum = 0;
    for (int64_t i = 0; i < NUM_TRIALS; ++i) {
        auto allone = UINT64_C(0xffffffffffffffff);
        while (allone) {
            sum += __builtin_ctzll(allone);
            allone &= allone - 1;
        }
    }
    auto end = std::chrono::system_clock::now();
    auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(end - start).count();
    std::cout << "u64 bench" << std::endl;
    std::cout << "elapsed: " << elapsed << " [msec]" << std::endl;
    if (elapsed != 0) {
        std::cout << "times/s: " << NUM_TRIALS * 1000 / elapsed << " [times/sec]" << std::endl;
        std::cout << "sum: " << sum << std::endl;
    }

    start = std::chrono::system_clock::now();
    sum = 0;
    for (int64_t i = 0; i < NUM_TRIALS; ++i) {
        Bitboard allone = ALL;
        while (allone) {
            auto sq = allone.pop_lsb_unchecked();
            sum += (uint64_t)sq;
        }
    }
    end = std::chrono::system_clock::now();
    elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(end - start).count();
    std::cout << "bitboard bench" << std::endl;
    std::cout << "elapsed: " << elapsed << " [msec]" << std::endl;
    if (elapsed != 0) {
        std::cout << "times/s: " << NUM_TRIALS * 1000 / elapsed << " [times/sec]" << std::endl;
        std::cout << "sum: " << sum << std::endl;
    }
}
