/// Returns the number of steps required to reach 1 in the Collatz sequence starting from `n`.
pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut n = n;
    let mut steps = 0;

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        steps += 1;
    }

    Some(steps)
}
