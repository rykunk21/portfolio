// Function to generate a dummy pixel sequence
pub fn generate_pixel_sequence() -> String {
    // Simple example: a 10x10 grid of 'X' characters
    let mut sequence = String::new();
    for _ in 0..10 {
        for _ in 0..10 {
            sequence.push('X');
        }
        sequence.push('\n');
    }
    sequence
}
