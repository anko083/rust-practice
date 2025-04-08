fn main() {
    // Розміри конверта
    const W: usize = 30; // ширина
    const H: usize = 15; // висота

    // Перевірка на валідність розміру
    if W < 10 || W > 80 || H < 10 || H > 80 {
        println!("Розміри повинні бути в діапазоні від 10 до 80!");
        return;
    }

    // Малюємо верхню частину конверту
    for i in 0..H / 2 {
        for j in 0..W {
            if j == i || j == W - i - 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    // Малюємо середину конверту (тіло)
    for _ in 0..W {
        print!("*");
    }
    println!();

    // Малюємо нижню частину конверту
    for i in (0..H / 2).rev() {
        for j in 0..W {
            if j == i || j == W - i - 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
