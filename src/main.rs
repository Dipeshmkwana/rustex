#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

mod heap;
mod comb_lock;

use std::mem;


fn stack_heap(){
    // stack: memory on which push/take out in LIFI 
    // heap: long term memory, dynamix, unfixed size cloud be store there

}


//constants
const MEANING_OF_LIFE:u8 = 42; // no fixed address
static mut Z:i32 = 123;

//ch#4
fn scope_and_shadowing(){
  let a = 123;
  let a = 1234; // this will print updated value

  {
      let b = 456;
      println!("inside, b = {}", b);
      let a = 777;
  }

  println!("outside, a = {}", a);

  //println!("outside ,b = {}", b);
}


//ch#3
fn operators(){

  //arithmetic
  let mut a = 2+3*4;  // +-*/
  println!("{}",a);

  a = a+1; // dont have -- ++
  a -= 2; // a = a -2;
            // -= += *= /= %=
  println!("reminder of {} / {} = {}", a, 3, (a%3));

  let a_cubed = i32::pow(a,3);
  println!(" {} cubed is {}", a, a_cubed);

  let b = 2.5;
  let b_cubed = f64::powi(b,3);
  let b_to_pi = f64::powf(b, std::f64::consts::PI);
  println!("{} cubed = {}, {}^pi = {}",b,b_cubed, b,b_to_pi);

  //bitwise 
  let c =1 | 2; // | OR & AND ^ XOR ! NOR
                // 01 OR 10 = 11 == 3_10

  println!("1|2 + {}",c);
  let two_to_10 = 1 << 10;  // >>
  println!("2^10 = {}", two_to_10);

  // logical 
  let pi_less_4 = std::f64::consts::PI < 4.0; // true
  println!("pi < 4 {}", pi_less_4);
  // > <= >= ==
  let x = 5;
  let x_is_5 = x == 5; // true
  println!("x is 5?  {}", x_is_5);

}

//ch#2
fn fundamental_data_types(){
  let a: u8 = 123; // u = unsigned, 8bit, 0-255
  println!("a = {}",a); // immutable

  // u = unsinged 0 to 2^N-1
  // i = signed, -2^(N-1) .. 2^(N-1)-1
  let mut b:i8=0; // -128 - 127
  println!("b = {} before", b);
  b = 42;
  println!("b = {} after", b);

  let c = 123456789;
  println!("c ={}, takes up {} bytes",c,mem::size_of_val(&c));

  //u8, u16, u32, u64, i8, i16, i32

  // usize isize
  // it will take size of CPU native size 64/32

  let z:isize = 123;
  let size_of_z = mem::size_of_val(&z);
  println!("z ={}, takes up {} bytes, {}-bit OS",
           z, size_of_z, size_of_z*8);

  let d: char = 'x';
  println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));

  // f32 f64 IEEE754 signed

  let e: f32 = 2.5;
  println!("{}, size = {} bytes", e, mem::size_of_val(&e));

}

fn main() {

  fundamental_data_types();
  operators();
  scope_and_shadowing();

  unsafe
  {
    Z = 777;
    println!("{} {}",Z,MEANING_OF_LIFE);
  }

  stack_heap();

  heap::stack_and_heap();
  comb_lock::comb_lock();

}
