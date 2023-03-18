//Highly Composite Number finder
//Reference: An Algorithm for Computing Highly Composite Numbers by Kiran S. Kedlaya, 2018
//https://shreevatsa.github.io/site/assets/hcn/hcn-algorithm.pdf

//Many of the function names in this program refer to the algorithms provided by Kedlaya 

use clap::Parser;
use lazy_static::lazy_static;

#[allow(non_upper_case_globals)]
#[allow(unconditional_recursion)]

#[derive(Parser)]
struct Args {
    n: i32,
}

//Define the first 20 primes here for convenient reference later. No reason to calculate these from scratch 
lazy_static! {
    static ref primes: Vec<i32> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71];
}

//Finds the number of divisors
fn tau(n: i32 ) -> i32 {
    println!("tau");
    let mut d: i32 = 1;

    for i in 1..n {
        if n % i == 0 {
            d += 1;
        }
    }
    
    return d; 
}

fn d(k: i32, n: i32) -> i32 {
    println!("d");
    let (g, h) = hcp(k, n);
    let d = (g + 1) * d(k - 1, h);

    return d;
}

fn f(k: i32, n: i32) -> i32 {
    println!("f");
    let (g, h) = hcp(k, n);
    let f: i32 = primes[k as usize].pow(g as u32) * f(k - 1, h);

    return f;
}

//Highly Composite Product, Algorithm 1
fn hcp(k: i32, n: i32) -> (i32, i32) {
    println!("hcp");
    println!("k: {}", k);
    let mut g: i32 = 1;
    let mut h: i32 = 1;
    let mut j: i32;
    let mut r: i32;
    let mut e: i32 = 1;
    let mut m: i32 = 1;

    if n == 1 {
        return (g, h);
    } else {
        r = 2 * f(k, n - 1);
        j = 1;
    }

    for s in (0..n).rev() {
        let a = (j + 1) * d(k - 1, s);
        let b = d(k, n - 1);
        let c = primes[k as usize].pow(j as u32);

        if a > b || c > r {
            if c <= r {
                r = primes[k as usize].pow(j as u32) * f(k - 1, s);
                e = j;
                m = s;
            }

            if s > 1 {
                j += 1;
            } else {
                //Return g(k, n) = e, h(k, n) = m
                return (e, m);
            }
        }
    }

    //If nothing works, just return a pair of 1s
    //This should not return but it's here just in case
    return (g, h);
}

//Highly Composite Number, Algorithm 2
fn hcn(n: i32) -> i32 {
    println!("hcn");
    let mut r: i32;
    let mut k: i32 = 1;

    if n == 1 {
        return 1;
    } else {
        r = 2 * hcn(n - 1);
        //k = 1
    }

    for s in (0..n).rev() { 
        let a = d(k, s);
        let b = tau(r/2); //Really, this is the tau of H(n - 1)
        let c = f(k, s);

        if a > b || c >= r {
            if c < r {
                r = f(k, s);
            }

            if s > 1 {
                k += 1;
            } else {
                return r;
            }
        }

    }

    return r;
}

fn main() {
    let args = Args::parse();
    
    for i in 1..args.n {
        println!("{}", hcn(i));
    }
}