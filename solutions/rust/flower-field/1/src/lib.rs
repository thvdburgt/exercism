pub fn annotate(garden: &[&str]) -> Vec<String> {
    let contains_flower =
        |x: usize, y: usize| -> bool { garden[y].as_bytes().get(x) == Some(&b'*') };

    let mut result = Vec::new();
    for y in 0..garden.len() {
        let mut line_result = String::new();
        let line = garden[y];

        // TODO assert each length is equal 
        for (x, ch) in line.as_bytes().iter().enumerate() {
            if *ch == b'*' {
                line_result.push(char::from(*ch));
                continue;
            }

            let has_upper = y != 0;
            let has_right = x != line.len() - 1;
            let has_lower = y != garden.len() - 1;
            let has_left = x != 0;

            let mut count = 0;
            if has_upper && has_left && contains_flower(x - 1, y - 1) {
                count += 1
            }
            if has_upper && contains_flower(x, y - 1) {
                count += 1
            }
            if has_upper && has_right && contains_flower(x + 1, y - 1) {
                count += 1
            }

            if has_left && contains_flower(x - 1, y) {
                count += 1
            }
            if has_right && contains_flower(x + 1, y) {
                count += 1
            }

            if has_lower && has_left && contains_flower(x - 1, y + 1) {
                count += 1
            }
            if has_lower && contains_flower(x, y + 1) {
                count += 1
            }
            if has_lower && has_right && contains_flower(x + 1, y + 1) {
                count += 1
            }

            let square_char = if count == 0 {
                ' '
            } else {
                char::from_digit(count, 10).expect("count is always <= 8")
            };
            line_result.push(square_char);
        }
        result.push(line_result);
    }

    result
}
