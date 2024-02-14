enum Option2<T> {
    None,
    Some(T),
}

impl<T> Option2<T> {
    fn is_some(&self) -> bool {
        return match self {
            Option2::None => false,
            Option2::Some(_) => true,
        };
    }
}

fn main() {
    let foo = Some(5);

    if foo.is_some() {
        let values = foo.unwrap();
    }
}
