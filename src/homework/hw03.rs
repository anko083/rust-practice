fn main() {
    // Розміри конверта
    const W: usize = 30; // ширина
    const H: usize = 15; // висота

    // Верхня частина конверта
    for i in 0..H/2 {
        for j in 0..W {
            if j == i || j == W - i - 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    // Тілова частина конверта
    for i in 0..H/2 {
        for _ in 0..W {
            print!("*");
        }
        println!();
    }
}
