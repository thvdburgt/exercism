pub fn is_armstrong_number(num: u32) -> bool {
    let digits = {
        let mut digits = Vec::new();
        let mut num = num;
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits
    };

    let number_of_digits = digits.len() as u32;
    let sum = digits.iter().map(|d| d.pow(number_of_digits)).sum();

    num == sum
}
