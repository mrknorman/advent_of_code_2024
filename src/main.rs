mod one;
mod two;
mod three;
mod four;

fn main(){
    //10.2 ms ±  12.9 ms 
    one::one_a();
    //4.8 ms ±   6.4 ms
    one::one_b();

    // 6.7 ms ± 4.2 ms 
    two::two_a();
    // 6.4 ms ± 3.9 ms
    two::two_b();

    //4.8 ms ± 2.3 ms 
    three::three_a();
    //4.9 ms ± 0.5 ms
    three::three_b();

    //7.9 ms ± 11.9 ms
    four::four_a();
    //8.3 ms ± 11.5 ms 
    four::four_b();
}