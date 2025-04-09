fn swap_case(input: &str) -> String {
    input.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else {
                c.to_string()  // Якщо символ не літера, залишаємо його без змін
            }
        })
        .collect()
}

fn main() {
    let input = "Добрий День!";  // Вхідний рядок
    let result = swap_case(input);  // Зміна регістру
    println!("Змінений регістр: {}", result);
}
