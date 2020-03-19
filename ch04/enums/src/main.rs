#[derive(PartialEq)]
enum Animal {
    Dog,
    Cat,
}

enum Relationship {
    Father,
    Mother,
    Daughter,
    Son,
    Sibling,
    Other(u32),
}

struct Foo {
    a: i32,
}

struct Bar {
    b: Foo
}

enum Baz {
    VarA(Foo),
    VarB(Bar),
}

fn main() {
    let my_pet = Animal::Dog;
    let other_pet = Animal::Cat;

    assert!(my_pet != other_pet);
}
