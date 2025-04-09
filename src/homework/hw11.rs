use rand::Rng; // Для генерації випадкових чисел

// Функція для генерації випадкового вектора довжиною `n`
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| rng.gen_range(10..100)) // Генерація випадкових чисел в діапазоні [10..99]
        .collect()
}

// Функція для знаходження мінімальної пари сум сусідніх елементів
fn min_adjacent_sum(data: &[i32]) -> (i32, usize) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_sum, min_index)
}

// Функція для виведення результату в консоль
fn print_results(data: &[i32], min_sum: i32, min_index: usize) {
    println!("Вектор: {:?}", data);
    println!("Мінімальна сума сусідніх елементів: {}", min_sum);
    println!("Індекси елементів, що дають мінімальну суму: {} і {}", min_index, min_index + 1);
}

fn main() {
    let n = 20;
    let random_vector = gen_random_vector(n);
    let (min_sum, min_index) = min_adjacent_sum(&random_vector);
    print_results(&random_vector, min_sum, min_index);
}
