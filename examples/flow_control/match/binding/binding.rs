// Функция под названием `age`, которая возвращает `u32`.
fn age() -> u32 {
    15
}

fn main() {
    println!("Скажите мне, к какой возрастной категории вы относитесь?");

    match age() {
        0             => println!("Мне кажется, что я ещё не родился..."),
        // Можно использовать `match` для конкретного значения в пределе 1 ... 12, но
        // как тогда определять возраст ребёнка, например? Вместо этого, свяжем `n` с
        // последовательностью от 1 до 12. Теперь мы можем сообщить о возрасте.
        n @ 1  ... 12 => println!("Я ребёнок! Мне {:?} лет.", n),
        n @ 13 ... 19 => println!("Я подросток. Мне {:?} лет.", n),
        // Больше пределов нет. Возвращаем результат.
        n             => println!("Я уже довольно старый, мне {:?}.", n),
    }
}
