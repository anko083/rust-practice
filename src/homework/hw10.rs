fn is_palindrome(n: i32) -> bool {
    let s = n.to_string(); // Перетворюємо число в рядок
    s == s.chars().rev().collect::<String>() // Перевіряємо, чи однакові початковий і перевернутий рядок
}

fn main() {
    let num = 12321;
    if is_palindrome(num) {
        println!("{} є паліндромом", num);
    } else {
        println!("{} не є паліндромом", num);
    }
}
