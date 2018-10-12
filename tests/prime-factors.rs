extern crate prime_factors;

use prime_factors::factors;
use prime_factors::PrimeFactory;

#[test]
fn test_1_is_prime() {
	assert!(PrimeFactory::is_prime(1));
}

#[test]
fn test_2_is_prime() {
	assert!(PrimeFactory::is_prime(2));
}

#[test]
fn test_11_is_prime() {
	assert!(PrimeFactory::is_prime(11));
}

#[test]
fn test_calc_first_6_primes() {
	let mut p = PrimeFactory::new();
	assert_eq!(p.nth_prime(0), 1);
	assert_eq!(p.nth_prime(1), 2);
	assert_eq!(p.nth_prime(2), 3);
	assert_eq!(p.nth_prime(3), 5);
	assert_eq!(p.nth_prime(4), 7);
	assert_eq!(p.nth_prime(5), 11);
}

#[test]
fn test_no_factors() {
    assert_eq!(factors(1), vec![]);
}

#[test]
fn test_prime_number() {
    assert_eq!(factors(2), vec![2]);
}

#[test]
fn test_square_of_a_prime() {
    assert_eq!(factors(9), vec![3, 3]);
}

#[test]
fn test_cube_of_a_prime() {
    assert_eq!(factors(8), vec![2, 2, 2]);
}

#[test]

fn test_product_of_primes_and_non_primes() {
    assert_eq!(factors(12), vec![2, 2, 3]);
}

#[test]

fn test_product_of_primes() {
    assert_eq!(factors(901255), vec![5, 17, 23, 461]);
}

#[test]
fn can_find_large_prime() {
	let mut p = PrimeFactory::new();
	let mut idx = 1;
	while p.nth_prime(idx) < 894119 {
		println!("{}", idx);
		idx += 1;
	}
	assert_eq!(p.nth_prime(idx), 894119);
}

#[test]
#[ignore]
fn test_factors_include_large_prime() {
    assert_eq!(factors(93819012551), vec![11, 9539, 894119]);
}
