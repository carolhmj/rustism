pub fn factors(n: u64) -> Vec<u64> {
    let mut factor_v : Vec<u64> = vec![];
   	let mut prime_factory = PrimeFactory::new();
   	let mut n = n;
   	let mut prime_idx = 1;

    while n != 1 {
    	println!("prime_idx {}", prime_idx); 
    	let p = prime_factory.nth_prime(prime_idx);
    	let rem = n % p;
    	if rem == 0 {
    		factor_v.push(p);
    		n = n / p;
    	} else {
    		prime_idx += 1;
    	}
    }

    factor_v
}

/// This is a prime numbers memoization factory
#[derive(Debug)]
pub struct PrimeFactory {
	calc_primes : Vec<u64>,
}

impl PrimeFactory {
	/// Constructs a new instance of the PrimeFactory type
	/// Ideally, this should be a singleton, but let's work with simple
	/// struct for now.
	/// 1 is added as the 0-th prime number because it's a easy way
	/// to start with a invariant: the prime numbers vector is never 
	/// empty
	pub fn new() -> PrimeFactory {
		let calc_primes = vec![1];
		PrimeFactory { calc_primes, }
	}

	pub fn nth_prime(&mut self, n : usize) -> u64 {
		if n > self.get_last_index() {
			self.calc_until(n);
		}
		self.calc_primes[n]		
	}

	// Calcs all primes from our last calculated one until (incl) n
	fn calc_until(&mut self, n : usize) {
		let mut search_number = (&self).get_last_prime();
		
		while self.get_last_index() <= n {
			search_number += 1;
			if PrimeFactory::is_prime(search_number) {
				self.calc_primes.push(search_number);
			}			
		}
	}

	/// Calculates the next prime of the sequence
	pub fn get_next_prime(&mut self) -> u64 {
		let last_index = self.get_last_index(); 
		self.nth_prime(last_index+1)
	}

	/// Returns the last prime that was calculated
	pub fn get_last_prime(&self) -> u64 {
		self.calc_primes[self.get_last_index()]
	}

	pub fn get_last_index(&self) -> usize {
		self.calc_primes.len()-1	
	}

	//is_prime is made pub because I wanted to test it, is there
	//any way to test non-pub functions?
	pub fn is_prime(i : u64) -> bool {
		!(2..i).into_iter().any(|x| i % x == 0)
	}
}

