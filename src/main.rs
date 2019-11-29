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
    second::float_type()
}

fn main2_3() {
    use dive_into_rust_of_fcc::ch02::third;
    third::learn_tuple();
    third::learn_struct();
    third::learn_tuple_struct();
    third::learn_enum();
}

fn main3_1() {
    use dive_into_rust_of_fcc::ch03::first;
    first::learn_operator();
    first::learn_bit_op();
    first::learn_bool_op();
    first::learn_assign();
    first::learn_statement_block();
}

fn main3_2() {
    use dive_into_rust_of_fcc::ch03::second;
    second::learn_if_else();
    second::learn_loop();
    second::learn_break_continue();
    second::learn_while();
    second::learn_for();
}

fn main() {
    main3_2();
}
