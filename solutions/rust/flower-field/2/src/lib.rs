pub fn annotate(garden: &[&str]) -> Vec<String> {
    let is_flower = |ch: u8| ch == b'*';
    let contains_flower = |x: usize, y: usize| -> bool { is_flower(garden[y].as_bytes()[x]) };

    #[rustfmt::skip]
    const ADJENCENCY_OFFSETS: &[(isize, isize)] = &[
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    if garden.is_empty() {
        return Vec::new();
    }

    let height = garden.len();
    let width = garden
        .first()
        .expect("Garden is non-empty")
        .as_bytes()
        .len();

    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    if contains_flower(x, y) {
                        return '*';
                    }

                    let flower_count = ADJENCENCY_OFFSETS
                        .iter()
                        // Calculate the index of all possible adjacent squares
                        .map(|&(x_offset, y_offset)| {
                            (
                                x.checked_add_signed(x_offset),
                                y.checked_add_signed(y_offset),
                            )
                        })
                        // Only keep those where neither index is < 0
                        .filter_map(|(x, y)| x.zip(y))
                        // Only keep those where neither index is not too large to be on the board
                        .filter(|&(x, y)| x < width && y < height)
                        // Only keep those that contain a flower
                        .filter(|&(x, y)| contains_flower(x, y))
                        .count();

                    if flower_count == 0 {
                        ' '
                    } else {
                        char::from_digit(flower_count as u32, 10)
                            .expect("There are max 8 adjecent squares")
                    }
                })
                .collect()
        })
        .collect()
}
