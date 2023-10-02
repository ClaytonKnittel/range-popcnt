/// Returns the maximum value of x.count_ones() for x in range_min..range_max.
pub fn range_popcnt(range_min: u64, range_max: u64) -> u32 {
  debug_assert!(range_min < range_max);
  let different_bits = range_min ^ range_max;
  let highest_different_bit = 63 - different_bits.leading_zeros();
  let y = (1u64 << highest_different_bit) - 1;
  (y | range_min).count_ones()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn range_popcnt_naive(range_min: u64, range_max: u64) -> u32 {
    (range_min..range_max)
      .map(|x| x.count_ones())
      .max()
      .unwrap()
  }

  #[test]
  fn test_range_popcnt() {
    for range_max in 1..64 {
      for range_min in 0..range_max {
        assert_eq!(
          range_popcnt(range_min, range_max),
          range_popcnt_naive(range_min, range_max)
        );
      }
    }
  }
}
