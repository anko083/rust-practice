fn draw_tree(triangles: usize) {
    let height = 5;  // Висота кожного трикутника
    let width = height * 2 - 1; // Ширина найбільшого трикутника

    // Малюємо кожний трикутник
    (1..=triangles).for_each(|i| {
        (0..height).for_each(|j| {
            let stars = 2 * j + 1; // Кількість зірочок на поточному рівні
            let spaces = (width - stars) / 2; // Кількість пробілів зліва від зірочок

                    // Виводимо рядок з пробілами та зірочками
                    (0..spaces).for_each(|_| print!(" "));
                    (0..stars).for_each(|_| print!("*"));
                    println!(); // Перехід на наступний рядок
                });
            });
        
            // Малюємо стовпчик ялинки
            (0..2).for_each(|_| {
                (0..(width / 3)).for_each(|_| print!(" "));
                (0..(width / 3)).for_each(|_| print!("*"));
                println!();
            });
        }
        
        fn main() {
            let triangles = 3; // Кількість трикутників для ялинки
            draw_tree(triangles);
        }