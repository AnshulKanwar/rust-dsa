fn print_matrix(matrix: &Vec<Vec<u32>>) {
    for row in matrix {
        for col in row {
            print!("{} ", col);
        }
        println!();
    }
}

pub fn matrix_chain_multiplication(d: &[u32]) -> u32 {
    let n = d.len() - 1;
    let mut m = vec![vec![0; n]; n];
    let mut s = vec![vec![0; n]; n];

    for x in 1..n {
        let mut i = 0;
        let mut j = i + x;
        loop {
            let mut smallest = 1000000;
            for k in i..j {
                let answer = m[i][k] + m[k + 1][j] + d[i] * d[k + 1] * d[j + 1];
                if answer < smallest {
                    smallest = answer;
                    m[i][j] = smallest;
                    s[i][j] = (k + 1) as u32;
                }
            }

            i += 1;
            j += 1;

            if i >= n || j >= n {
                break;
            }
        }
    }

    // let answer = m[i][k] + m[k + 1][j] + d[i] * d[k + 1] * d[j + 1];
    // printing
    println!("Matrix m is:");
    print_matrix(&m);
    println!("\n\n");
    println!("Matrix s is:");
    print_matrix(&s);

    return 0;
}
