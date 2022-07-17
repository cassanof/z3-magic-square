use std::ops::{Add, Mul};

use z3::{
    ast::{Array, Ast, Bool, Int},
    Config, Context, Solver,
};

fn main() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        println!("Usage: {} <size> <magic number>", args[0]);
        std::process::exit(1);
    }

    let size: i64 = args[1].parse().unwrap();
    let magic_num: i64 = args[2].parse().unwrap();

    let z3_magic_num = Int::from_i64(&ctx, magic_num);
    let mut sq_arr: Vec<Vec<Int>> = vec![];

    for i in 0..size {
        let mut row_arr: Vec<Int> = vec![];
        for j in 0..size {
            let (i, j) = (i as i64, j as i64);
            let num = Int::new_const(&ctx, format!("{}_{}", i, j));
            // has to be positive and non-zero, and less than or equal to 9
            solver.assert(&Bool::and(
                &ctx,
                &[
                    &num.gt(&Int::from_i64(&ctx, 0)),
                    &num.le(&Int::from_i64(&ctx, 9)),
                ],
            ));
            row_arr.push(num);
        }
        sq_arr.push(row_arr);
    }

    for row in sq_arr.iter() {
        let mut acc = Int::from_i64(&ctx, 0);
        for num in row.iter() {
            acc += num;
        }
        solver.assert(&acc._eq(&z3_magic_num));
    }

    for i in 0..size {
        let mut col: Vec<&Int> = vec![];
        for j in 0..size {
            col.push(&sq_arr[j as usize][i as usize]);
        }

        let mut acc = Int::from_i64(&ctx, 0);
        for num in col.iter() {
            acc += *num;
        }
        solver.assert(&acc._eq(&z3_magic_num));
    }

    // now the diagonals
    {
        let mut diag_acc = Int::from_i64(&ctx, 0);
        for i in 0..size {
            diag_acc += &sq_arr[i as usize][i as usize];
        }
        solver.assert(&diag_acc._eq(&z3_magic_num));

        let mut diag_acc = Int::from_i64(&ctx, 0);
        for i in 0..size {
            diag_acc += &sq_arr[i as usize][(size - i - 1) as usize];
        }
        solver.assert(&diag_acc._eq(&z3_magic_num));
    }

    println!("SIZE: {}", size);
    println!("MAGIC_NUM: {}", magic_num);
    match solver.check() {
        z3::SatResult::Unsat => {
            println!("unsat");
        }
        z3::SatResult::Unknown => {
            println!("unknown");
        }
        z3::SatResult::Sat => {
            let model = solver.get_model();
            if let Some(model) = model {
                let mut buf = String::new();
                for row in sq_arr.iter() {
                    #[allow(clippy::format_push_string)]
                    for cell in row.iter() {
                        let evalled = model.eval(cell, false).unwrap().as_i64().unwrap();
                        buf.push_str(&format!("{} ", evalled));
                    }
                    buf.push('\n');
                }
                println!("{}", buf);
            }
        }
    }
}
