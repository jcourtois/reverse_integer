#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = x.signum();
        let reversed = x.abs().to_string().bytes().rev().collect();
        match String::from_utf8(reversed).unwrap().parse::<i32>() {
            Ok(num) => sign * num,
            Err(_) => 0,
        }
    }

    #[cfg(test)]
    pub fn reverse2(x: i32) -> i32 {
        let sign = x.signum();
        let mut n = x.abs();
        let mut ans = Some(0);
        let mut temp;
        while n > 0 {
            temp = n % 10;
            ans = ans
                .and_then(|x: i32| x.checked_mul(10i32))
                .and_then(|x: i32| x.checked_add(temp));
            n /= 10;
        }
        ans.map_or(0, |x| x * sign)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _0_is_0() {
        assert_eq!(Solution::reverse(0), 0);
    }

    #[test]
    fn _1_is_1() {
        assert_eq!(Solution::reverse(1), 1);
    }

    #[test]
    fn _2_is_2() {
        assert_eq!(Solution::reverse(2), 2);
    }

    #[test]
    fn _neg_1_is_neg_1() {
        assert_eq!(Solution::reverse(-1), -1);
    }

    #[test]
    fn _21_is_21() {
        assert_eq!(Solution::reverse(12), 21);
    }

    #[test]
    fn _123_is_321() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn _neg_123_is_321() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn _120_is_21() {
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn _positive_overflow() {
        assert_eq!(Solution::reverse(2147483647), 0);
    }

    #[test]
    fn _negative_overflow() {
        assert_eq!(Solution::reverse(2147483647), 0);
    }
}
