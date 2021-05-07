fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    // Was going to use println! until I realized the ast api
    // for macros is complicated. so I'm just using function calls.
    print("Hello World!");

    print(2 + 2, 3 / 2);

    let result = 3 + 5 * 2;
    print(result);

    let i = 0;
    loop {
        // i += 1;
        i = i + 1;

        if i > 5 {
            break;
        }
    }
}
