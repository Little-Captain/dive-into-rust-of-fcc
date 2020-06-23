#![allow(dead_code)]

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

fn main4_1() {
    use dive_into_rust_of_fcc::ch04::first;
    first::first();
    first::test_inner();
    // first::learn_diverging_func();
    first::learn_main_fn();
    first::learn_const_fn();
    first::learn_recursion();
}

fn main5_1() {
    use dive_into_rust_of_fcc::ch05::eighth;
    use dive_into_rust_of_fcc::ch05::fifth;
    use dive_into_rust_of_fcc::ch05::first;
    use dive_into_rust_of_fcc::ch05::fourth;
    use dive_into_rust_of_fcc::ch05::second;
    use dive_into_rust_of_fcc::ch05::sixth;
    use dive_into_rust_of_fcc::ch05::third;
    first::trait1();
    first::trait2();
    first::trait3();
    second::learn_static_method();
    third::learn_extension();
    fourth::learn_call_func();
    fourth::learn_call_func1();
    fifth::learn_trait1();
    sixth::learn_derive();
    eighth::learn_display_debug();
    eighth::learn_float();
}

fn main6_1() {
    use dive_into_rust_of_fcc::ch06::first;
    use dive_into_rust_of_fcc::ch06::second;
    first::learn_array1();
    first::learn_array2();
    first::learn_array3();
    first::learn_array4();
    first::learn_array5();
    first::learn_array6();
    first::learn_array7();
    first::learn_array8();
    first::learn_array9();
    first::learn_array10();
    first::learn_array11();
    second::learn_str1();
    second::learn_string1();
    second::learn_string2();
}

fn main7_1() {
    use dive_into_rust_of_fcc::ch07::first;
    use dive_into_rust_of_fcc::ch07::second;
    use dive_into_rust_of_fcc::ch07::third;
    first::destructure1();
    first::destructure2();
    second::first();
    second::second();
    second::third();
    second::fourth();
    second::fifth();
    second::sixth();
    second::seventh();
    third::first();
    third::second();
}

fn main_8() {
    use dive_into_rust_of_fcc::ch08::first;
    // first::first();
    // first::second();
    first::third();
}

fn main_9() {
    use dive_into_rust_of_fcc::ch09::first;
    first::first();
    first::second();
    first::third();
}
fn main_11() {
    use dive_into_rust_of_fcc::ch11::first;
    use dive_into_rust_of_fcc::ch11::second;
    first::first();
    second::first();
    second::second();
    second::third();
    second::fourth();
    second::fifth();
    second::sixth();
}

fn main_11_1() {
    use dive_into_rust_of_fcc::ch11::third;
    third::first();
    third::second();
}

fn main_12() {
    use dive_into_rust_of_fcc::ch12::first;
    first::first();
    first::second();
    first::third();
    first::fourth();
}

fn main_13() {
    use dive_into_rust_of_fcc::ch13::first;
    first::first();
    first::second();
    first::third();
    first::fourth();
    first::fifth();
}

fn main_14() {
    use dive_into_rust_of_fcc::ch14::first;
    first::first();
    first::second();
}

fn main_15() {
    use dive_into_rust_of_fcc::ch15::first;
    first::first();
    first::second();
    first::third();
}

fn main_16() {
    use dive_into_rust_of_fcc::ch16::first;
    first::first();
    first::second();
    first::third();
    first::fourth();
}

fn main() {
    main_16();
}
