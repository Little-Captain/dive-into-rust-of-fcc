#[allow(dead_code)]

fn main1() {
    use dive_into_rust_of_fcc::ch01::first;
    first::hello_world();
    first::print();
}

fn main2_1() {
    use dive_into_rust_of_fcc::ch02::first;
    first::def_variable();
    first::shadowing_var();
    first::shadowing_var1();
    first::infer_type();
    first::infer_type2();
    first::type_alias();
    first::use_global();
    first::const_var();
}

fn main2_2() {
    use dive_into_rust_of_fcc::ch02::second;
    second::learn_bool();
    second::learn_char();
    second::integer_type();
    second::integer_overflow();
}

fn main() {
    main2_2();
}
