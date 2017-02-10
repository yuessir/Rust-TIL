use std::io::Write;
use std::str::FromStr;
/* Rust Lang  TIL*/
fn main() {
    //println!("Hello, HuayInThon!");
    let mut number=Vec::new();
 
   /* for arg in std::env::args().take(1){ //std::ienv::args returns a iterator.
        println!("The program name is {}",arg);
    }*/

   for arg in std::env::args().skip(1) {
      number.push(u64::from_str(&arg).expect("error parsing the argument!"));
   }
   if number.len()==0 {
       writeln!(std::io::stderr(),"no argument be inputted, exiting..");
       std::process::exit(1);// by explicitly calling functions to terminate the program with the error code
   }
   
}


/*
 *Computing the greatest common divisor of two integers.
*/
fn gcd(mut n:u64,mut m :u64)->u64{//mut is ketword,short for 'mutable' 
    assert!(n!=0&&m!=0);
    while m!=0 {
        if m<n {
            let t:u64=m;//explicitly type
            m=n;
            n=t;
        }
        m=m%n;
    }
    n
}

#[test]
fn gcd_test() {
    assert_eq!(gcd(3,11),1 );
   // assert_eq!(gcd(5,10),1 );//be failure
    assert_eq!(gcd(5,10),5 );
}
