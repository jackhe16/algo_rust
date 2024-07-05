//! # 线性筛选 (欧拉质数筛选)
//!
//!

pub struct LinearSieve {}

impl LinearSieve {
    pub fn linear_sieve(n: usize) -> Vec<usize> {
        let mut is_primes = vec![true; n + 1];
        let mut primes = vec![];

        for i in 2..n + 1 {
            if is_primes[i] {
                primes.push(i);
            }

            for &p in primes.iter() {
                if i * p > n {
                    break;
                }
                is_primes[i * p] = false;
                if i % p == 0 {
                    break;
                }
            }
        }

        primes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_sieve() {
        let res = LinearSieve::linear_sieve(20);
        println!("{:?}", res);
    }
}
