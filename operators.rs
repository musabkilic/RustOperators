extern crate eval;

use eval::{eval};

fn main(){
    for i in ["+", "-", ""].iter(){
        for j in ["+", "-", ""].iter(){
            for k in ["+", "-", ""].iter(){
                for l in ["+", "-", ""].iter(){
                    for m in ["+", "-", ""].iter(){
                        for n in ["+", "-", ""].iter(){
                            for o in ["+", "-", ""].iter(){
                                for p in ["+", "-", ""].iter(){
                                    let exp = format!("1{}2{}3{}4{}5{}6{}7{}8{}9", i, j, k, l, m, n, o, p);
                                    let result = eval(&exp).unwrap();
                                    if result == 100{
                                        println!("{}=100", exp);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
