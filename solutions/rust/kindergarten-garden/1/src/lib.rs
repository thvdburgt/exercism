/// Returns the plants assigned to a student based on the garden diagram.
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    // Determine index of given child in order
    let student_index = match student {
        "Alice" => 0,
        "Bob" => 1,
        "Charlie" => 2,
        "David" => 3,
        "Eve" => 4,
        "Fred" => 5,
        "Ginny" => 6,
        "Harriet" => 7,
        "Ileana" => 8,
        "Joseph" => 9,
        "Kincaid" => 10,
        "Larry" => 11,
        _ => panic!("Unknown student"),
    };

    // Split the diagram into lines and extract the plants for the given student
    diagram
        .lines()
        .flat_map(|line| {
            line.chars()
                .skip(2 * student_index)
                .take(2)
                .map(|c| match c {
                    'G' => "grass",
                    'C' => "clover",
                    'R' => "radishes",
                    'V' => "violets",
                    _ => panic!("Unknown plant character"),
                })
        })
        .collect::<Vec<_>>()
}
