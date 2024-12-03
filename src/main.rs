mod one;
mod two;
mod three;

fn main() {
    one::one_a();
    one::one_b();

    // 6.7 ms ± 4.2 ms 
    two::two_a();

    // 6.4 ms ± 3.9 ms
    two::two_b();

    three::three_a();
    three::three_b();
}