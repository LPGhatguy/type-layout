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

#[derive(TypeLayout)]
struct GenericStruct<T> {
    f: T,
}

#[derive(TypeLayout)]
struct TupleStruct(i32, i8, i32);

#[derive(TypeLayout)]
struct GenericTupleStruct<T>(i32, T, i32);

#[derive(TypeLayout)]
struct Empty;

fn main() {
    println!("{}", Foo::type_layout());
    println!("{}", Bar::type_layout());
    println!("{}", GenericStruct::<i32>::type_layout());
    println!("{}", TupleStruct::type_layout());
    println!("{}", GenericTupleStruct::<i8>::type_layout());
    println!("{}", Empty::type_layout());
}
