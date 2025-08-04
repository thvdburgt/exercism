const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

/// Returns the plants assigned to a student based on the garden diagram.
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    // Determine index of given child in order
    let student_index = STUDENTS
        .iter()
        .position(|&name| name == student)
        .expect("Unknown student");

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
