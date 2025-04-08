fn main() {
    const W: usize = 21; // Ширина ромба (має бути непарною для коректного вигляду)
    const H: usize = 11; // Висота ромба

    // Перевірка на валідність розміру
    if W < 10 || W > 80 || H < 10 || H > 80 {
        println!("Розміри повинні бути в діапазоні від 10 до 80!");
        return;
    }

    let mut result = String::new();

    // Малюємо верхню частину ромба
    for i in 0..H / 2 {
        for j in 0..W {
            if j == (W / 2 - i) || j == (W / 2 + i) {
                result.push('*');
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }

    // Малюємо середину ромба
    for j in 0..W {
        result.push('*');
    }
    result.push('\n');

    // Малюємо нижню частину ромба
    for i in (0..H / 2).rev() {
        for j in 0..W {
            if j == (W / 2 - i) || j == (W / 2 + i) {
                result.push('*');
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }

    // Виводимо результат
    print!("{}", result);
}
