use type_layout::TypeLayout;

#[repr(C)]
#[derive(TypeLayout)]
struct Foo {
    a: u8,
    b: u16,
}

#[derive(TypeLayout)]
struct Bar {
    a: u8,
    b: u16,
}

fn main() {
    println!("{}", Foo::type_layout());
    println!("{}", Bar::type_layout());
}
