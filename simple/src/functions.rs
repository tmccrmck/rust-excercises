pub mod functions {

    pub fn sum(slice: &[i32]) -> i32 {
        // TODO
        let mut x: i32 = 0;
       	for num in slice {
       		x = x + num;
       	}
       	x
    }

    /// Deduplicates items in the input vector `vs`. Produces a vector containing
    /// the first instance of each distinct element of `vs`, preserving the
    /// original order.
    pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
        let mut v: Vec<i32> = vec![];
        for num in vs {
        	if !v.contains(num){
        		v.push(*num);
        	}
        }
        v
    }

    /// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
    /// `bool`). Returns a new vector containing only elements that satisfy `pred`.
    pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
        let mut v: Vec<i32> = vec![];
        for num in vs {
        	if pred(*num){
        		v.push(*num);
        	}
        }
        v
    }

    /// Find all prime numbers less than `n`.
    /// For example, `sieve(7)` should return `[2, 3, 5]`
    pub fn sieve(n: u32) -> Vec<u32> {
        let mut primes: Vec<u32> = vec![];
        let mut seen: Vec<u32> = vec![];
        for x in 2..n {
        	if !seen.contains(&x) {
        		primes.push(x);
        		let mut i = x;
        		while i < n {
        			seen.push(i);
        			i = i + x;
        		}
        	}
        }
        primes
    }

    /// Solves for the sequence of moves required to move all discs from peg 1 to
    /// peg 3, using peg 2 as an intermediary.
    pub fn hanoi(num_discs: u32) -> Vec<(u8, u8)> {
        // TODO
        let mut moves = vec![];
        return hanoi_helper(num_discs, 1, 2, 3, &mut moves)
    }

    pub fn hanoi_helper(height: u32, from: u32, aux: u32, to: u32, moves: &mut Vec<(u8, u8)>) -> Vec<(u8, u8)> {
    	if height >= 1 {
    		hanoi_helper(height-1, from, to, aux, moves);
    		moves.push((from as u8, to as u8));
    		hanoi_helper(height-1, aux, to, from, moves);
    	}
    	return moves.clone()
    }

    /// Simulates a bloom filter by accepting an array of three hash functions, a
    /// data vector, and another value to query. Returns `true` if `value` is
    /// "probably" in the data vector and `false` if it is definitely not in the
    /// data vector.
    pub fn bloom(data: &Vec<&str>, hashes: [fn(&[u8]) -> u64; 3], value: &str) -> bool {
        // TODO
        let mut array: [u64; 20] = [0; 20];

        for word in data {
        	for hash in hashes.iter() {
        		let x:u64 = hash(word.as_bytes()) % 20;
        		array[x as usize] = 1;
        	}
        }

        for hash in hashes.iter() { 
        	let val = hash(value.as_bytes()) % 20;
        	if array[val as usize] == 0 {
        		return false;
        	}
        }

        return true;
    }

    pub fn djb2(bytes: &[u8]) -> u64 {
        let mut hash: u64 = 5381;
        for b in bytes {
            // hash * 33 + c
            hash = (hash.wrapping_shr(5) + hash) + (*b as u64);
        }

        return hash;
    }

    pub fn fnv(bytes: &[u8]) -> u64 {
        let mut hash = 0xcbf29ce484222325;
        for b in bytes {
            hash = hash ^ (*b as u64);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        return hash;
    }

    pub fn jenkins(bytes: &[u8]) -> u64 {
        let mut hash = 0;
        for b in bytes {
            hash += *b as u64;
            hash += hash.wrapping_shr(10);
            hash ^= hash.wrapping_shl(6);
        }
        hash += hash.wrapping_shr(3);
        hash ^= hash.wrapping_shl(11);
        hash += hash.wrapping_shr(15);
        return hash;
    }
}