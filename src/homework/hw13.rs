fn calculate_total_area(rectangles: &[(u32, u32)]) -> u32 {
    rectangles.iter().map(|&(length, width)| length * width).sum()
}

fn main() {
    let rectangles = vec![
        (10, 5), // Перший прямокутник
        (4, 6),  // Другий прямокутник
        (7, 3),  // Третій прямокутник
    ];

    let total_area = calculate_total_area(&rectangles);
    println!("Загальна площа: {}", total_area);
}
