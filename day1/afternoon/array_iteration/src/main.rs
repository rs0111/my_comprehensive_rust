fn main() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {

            // prime % i != 0
            assert_ne!(prime % i, 0);
        }
    }
}
