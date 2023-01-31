#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reversing_zero() {
        assert_eq!(Solution::reverse(0), 0);
    }
}