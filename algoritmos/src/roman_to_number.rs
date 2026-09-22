pub fn parse(roman_value: &str) -> i32 {
    let chars: Vec<char> = roman_value.chars().collect();
    let mut number = 0;

    for i in 0..chars.len() - 1 {
        let now = number_equivalent(chars[i]);
        let next = number_equivalent(chars[i + 1]);

        if now < next {
            number -= now;
        } else {
            number += now;
        }
    }
    number += number_equivalent(chars[chars.len() - 1]);

    number
}

fn number_equivalent(letter: char) -> i32 {
    match letter {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}