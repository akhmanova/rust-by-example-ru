fn main() {
    let n = 5;

    if n < 0 {
        print!("{} — отрицательное", n);
    } else if n > 0 {
        print!("{} — положительное", n);
    } else {
        print!("{} — нуль", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", малое по модулю число, умножим его в десять раз");

            // Это выражение вернет `i32`.
            10 * n
        } else {
            println!(", большое по модулю число, уменьшим его вдвое");

            // И это выражение вернет `i32`.
            n / 2
            // TODO ^ Попробуйте отброить значение, добавив точку с запятой.
        };
    //   ^ Не забудьте добавить тут точку с запятой! Все операторы `let` требуют её..

    println!("{} -> {}", n, big_n);
}
