extern crate eval;

use eval::{eval};

fn main(){
    let mut num = 0;
    let operator_list = [" + ", " - ", ""];
    for i in operator_list.iter(){
        for j in operator_list.iter(){
            for k in operator_list.iter(){
                for l in operator_list.iter(){
                    for m in operator_list.iter(){
                        for n in operator_list.iter(){
                            for o in operator_list.iter(){
                                for p in operator_list.iter(){
                                    let exp = format!("1{}2{}3{}4{}5{}6{}7{}8{}9", i, j, k, l, m, n, o, p);
                                    let result = eval(&exp).unwrap();
                                    if result == 100{
                                        println!("{} = 100", exp);
                                        num += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("\nFound {} solutions!!", num);
}
