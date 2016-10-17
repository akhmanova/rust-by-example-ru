fn main() {
    // Объявляем связь с переменной
    let a_binding;

    {
        let x = 2;

        // Инициализируем связь
        a_binding = x * x;
    }

    println!("связь а: {}", a_binding);

    let another_binding;

    // Ошибка! Использование неициализированной связи с переменной
    println!("другая связь: {}", another_binding);
    // FIXME ^ Закомментируйте строку

    another_binding = 1;

    println!("другая связь: {}", another_binding);
}
