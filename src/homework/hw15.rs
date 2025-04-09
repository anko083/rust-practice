use itertools::Itertools;

fn is_valid_solution(m: u32, u: u32, x: u32, a: u32, s: u32, l: u32, o: u32, n: u32) -> bool {
    // Формуємо числа
    let muxa = m * 1000 + u * 100 + x * 10 + a;
    let x_a = x * 10 + a;
    let slon = s * 1000 + l * 100 + o * 10 + n;

    // Перевіряємо чи виконується рівність
    muxa == slon / x_a
}

fn count_solutions() -> usize {
    let mut count = 0;

    // Генерація всіх перестановок чисел від 1 до 8 для літер
    for permutation in (1..=8).permutations(8) {
        let m = permutation[0];
        let u = permutation[1];
        let x = permutation[2];
        let a = permutation[3];
        let s = permutation[4];
        let l = permutation[5];
        let o = permutation[6];
        let n = permutation[7];

        // Перевіряємо чи задовольняє перестановка умови задачі
        if is_valid_solution(m, u, x, a, s, l, o, n) {
            count += 1;
            // Виводимо правильне рішення
            println!("Рішення: ");
            println!("{}{}{}{}", m, u, x, a);
            println!("{}        {}", x, a);
            println!("  ------");
            println!("    {}{}{}{}", s, l, o, n);
            println!();
        }
    }

    count
}

fn main() {
    let num_solutions = count_solutions();
    println!("Кількість рішень: {}", num_solutions);
}
