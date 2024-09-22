fn unpucking_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    let mut escape_mode = false; // Флаг для экранирования

    while let Some(c) = chars.next() {
        if escape_mode {
            result.push(c);
            escape_mode = false;
        } else if c == '\\' {
            escape_mode = true;
        } else if c.is_alphabetic() {
            result.push(c);
            if let Some(next_char) = chars.peek() {
                if next_char.is_digit(10) {
                    if let Some(digit) = chars.next().unwrap().to_digit(10) {
                        result.push_str(&c.to_string().repeat(digit as usize - 1));
                    }
                }
            }
        } else if c.is_digit(10) {
            // Если цифра, повторяем последний символ (или экранируем)
            if let Some(last_char) = result.chars().last() {
                result.push_str(&last_char.to_string().repeat(c.to_digit(10).unwrap() as usize - 1));
            } else {
                eprintln!("Ошибка: некорректная строка.");
                break
            }
        } else {
            result.push(c);
        }
    }

    if escape_mode {
        result.push('\\'); // добавляем слэш, если строка заканчивается экранированием
    }

      result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unpacking_string() {
        // Тесты для корректных строк
        assert_eq!(unpucking_string("a3"), "aaa");
        assert_eq!(unpucking_string("a4bc2d5e"), "aaaabccddddde");
        assert_eq!(unpucking_string(r"qwe\4\5"), "qwe45");
        assert_eq!(unpucking_string(r"qwe\45"), "qwe44444"); // экранирование слэша
        assert_eq!(unpucking_string(r"qwe\\5"), "qwe\\\\\\\\\\"); // добавлено для проверки
    }
}

fn main() {
    let x = unpucking_string("");
    println!("{:?}", x);
}
