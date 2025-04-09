fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false; // Число 1 і всі числа менше 1 не є простими
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false; // Якщо є дільник, то число не просте
        }
    }
    true // Число просте
}

fn main() {
    let number = 29; // Тестове число
    if is_prime(number) {
        println!("Число {} є простим!", number);
    } else {
        println!("Число {} не є простим.", number);
    }
}
