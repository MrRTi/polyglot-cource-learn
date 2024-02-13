enum RSEnum {
    Foo2(Option<i32>),
    Foo(i32),
    Bar(String),
    Baz(Vec<String>),
    Baz2(fn() -> i32),
}

fn baz() -> i32 {
    5
}

fn main() {
    let foo = RSEnum::Foo(5);

    if let RSEnum::Foo(value) = foo {}
    if let RSEnum::Baz2(value) = foo {}

    match foo {
        RSEnum::Foo2(Some(value)) => {}
        RSEnum::Foo2(None) => {}
        RSEnum::Foo(value) => {}
    }
}
