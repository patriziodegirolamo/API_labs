
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    //println!("{:?}", minefield);

    let mut board = Vec::new();
    let n_rows = minefield.len();

    for i in 0..n_rows{
        let mut row = String::new();
        //println!("n-esimo {}",);
        for (j, car) in minefield[i].chars().enumerate(){
            match car {
                '*' => {row.push('*')},
                _ => {
                    let sol = count_in_row(minefield, i as i32, j as i32, n_rows as i32);
                    if sol == 0{
                        row.push(' ');
                    }
                    else{
                        row.push_str(&*sol.to_string());
                    }

                }
            }
        }
        board.push(row);
    }
    return board;
}

pub fn count_in_row(minefield: &[&str], i:i32, j:i32, n_rows:i32) -> usize{
    let mut count = 0;
    if i-1 >= 0 {
        count = count +count_in_col(minefield, (i-1) as usize, j, false, n_rows);
    }

    if i+1 < n_rows {
        count = count +count_in_col(minefield, (i+1) as usize, j, false, n_rows);
    }

    count = count +count_in_col(minefield, i as usize, j, true, n_rows);
    return count;
}

pub fn count_in_col(minefield: &[&str], i:usize, j:i32, current:bool, n_rows: i32) -> usize{
    let mut count_col = 0;

    if j-1 >= 0{
        //println!("{}", minefield[i].chars().nth((j-1) as usize).unwrap());
        if minefield[i].chars().nth((j-1) as usize).unwrap().eq(&'*'){
            count_col = count_col +1;
        }
    }

    if j+1 < n_rows{
        //println!("{}", minefield[i].chars().nth((j+1) as usize).unwrap());
        if minefield[i].chars().nth((j+1) as usize).unwrap().eq(&'*'){
            count_col = count_col +1;
        }
    }

    if !current {
        //println!("{}", minefield[i].chars().nth(j as usize).unwrap());
        if minefield[i].chars().nth(j as usize).unwrap().eq(&'*'){
            count_col = count_col +1;
        }
    }
    return count_col;
}