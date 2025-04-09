fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s; // Повертаємо рядок без змін, якщо він порожній
    }

    // Обчислюємо коректний зсув, використовуючи модуль довжини рядка
    let n = ((n % len as isize) + len as isize) % len as isize; // Зсув з урахуванням від'ємних значень
    let n = n as usize; // Перетворення в позитивний тип для використання індексів

    // Розбиваємо рядок на дві частини і міняємо їх місцями
    let (left, right) = s.split_at(len - n);
    format!("{}{}", right, left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(
                rotate(s.clone(), *n), 
                exp.to_string()
            );
        });
    }
}

fn main() {
    // Запуск основної програми (якщо потрібно)
    let s = "abcdefgh".to_string();
    let shifted = rotate(s, 2);
    println!("Зсунутий рядок: {}", shifted);  // Виведе "ghabcdef"
}
