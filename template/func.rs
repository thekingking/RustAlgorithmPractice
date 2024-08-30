struct Utils;

const MOD: i64 = 1_000_000_007;


impl Utils {
    /// k个m相乘，适用于k较大时简化计算
    fn pow(mut x: i64, mut n: i32) -> i64 {
        let mut res = 1;
        while n > 0 {
            if n % 2 == 1 {
                res = res * x % MOD;
            }
            x = x * x % MOD;
            n /= 2;
        }
        res
    }

    /// count_bits
    pub fn count_set_bits(mut n: u32) -> u32 {
        // Initialize a variable to keep track of the count of set bits
        let mut count = 0;
        while n > 0 {
            // Clear the least significant set bit by
            // performing a bitwise AND operation with (n - 1)
            n &= n - 1;
    
            // Increment the count for each set bit found
            count += 1;
        }
    
        count
    }

    pub fn find_highest_set_bit(num: i32) -> Option<i32> {
        if num < 0 {
            // Input cannot be negative.
            panic!("Input cannot be negative");
        }
    
        if num == 0 {
            return None; // No bit is set, return None.
        }
    
        let mut position = 0;
        let mut n = num;
    
        while n > 0 {
            n >>= 1;
            position += 1;
        }
    
        Some(position - 1)
    }

    pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
        fn match_compare<T: Ord>(
            item: &T,
            arr: &[T],
            left: &mut usize,
            right: &mut usize,
            is_asc: bool,
        ) -> bool {
            let mid = *left + (*right - *left) / 2;
            let cmp_result = item.cmp(&arr[mid]);
        
            match (is_asc, cmp_result) {
                (true, Ordering::Less) | (false, Ordering::Greater) => {
                    *right = mid;
                }
                (true, Ordering::Greater) | (false, Ordering::Less) => {
                    *left = mid + 1;
                }
                (_, Ordering::Equal) => {
                    *left = mid;
                    return true;
                }
            }
        
            false
        }

        
        let is_asc = is_asc_arr(arr);
    
        let mut left = 0;
        let mut right = arr.len();
    
        while left < right {
            if match_compare(item, arr, &mut left, &mut right, is_asc) {
                return Some(left);
            }
        }
    
        None
    }


}