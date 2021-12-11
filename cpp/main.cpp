#include <primesieve.hpp>
#include <iostream>
#include <omp.h>

#define MAX 223092870 // 23#
//#define MAX 614889782588491410 // 47#
#define COEFFICIENT 6

int main()
{
    const uint64_t dist = MAX;

    const int threads = omp_get_max_threads();
    const uint64_t thread_dist = (dist / threads) + 1;

    int64_t max_qs[threads];
    int64_t min_ps[threads];
    uint64_t is[threads];

    #pragma omp parallel for
    for (int thread = 0; thread < threads; thread++) {
        const uint64_t start = thread*thread_dist;
        const uint64_t stop = std::min(start + thread_dist, dist);

        int64_t max_q = std::numeric_limits<int64_t>::min();
        int64_t min_p = 0;
        uint64_t i = 0; // The "offset" is added later (see max_q_corrected)

        primesieve::iterator it(start, stop);

        for (int64_t prime = it.next_prime(); prime <= stop; prime = it.next_prime()) {
            int64_t q = COEFFICIENT*i - prime + 3;

            if (q > max_q) {
                max_q = q;
                min_p = prime;
            }

            i++;
        }

        max_qs[thread] = max_q;
        min_ps[thread] = min_p;
        is[thread] = i;
    }

    int64_t max_q_total = max_qs[0];
    int64_t min_p_total = min_ps[0];

    int64_t total_i = 0;
    for (int thread = 0; thread < threads; thread++) {
        int64_t max_q_corrected = max_qs[thread] + COEFFICIENT*total_i;
    
        std::cout << "Thread " << thread << ":" << std::endl;
        std::cout << "  Max q:          " << max_qs[thread] << std::endl;
        std::cout << "  (corrected):    " << max_q_corrected << std::endl;
        std::cout << "  Min p:          " << min_ps[thread] << std::endl;
        std::cout << "  Primes counted: " << is[thread] << std::endl;

        if (max_q_corrected > max_q_total) {
            max_q_total = max_q_corrected;
            min_p_total = min_ps[thread];
        }

        total_i += is[thread];
    }

    std::cout << "Total:" << std::endl;
    std::cout << "  Max q:          " << max_q_total << std::endl;
    std::cout << "  Min p:          " << min_p_total << std::endl;
    std::cout << "  Primes counted: " << total_i << std::endl;

    return 0;
}

