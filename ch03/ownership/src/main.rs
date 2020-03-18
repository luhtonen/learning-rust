fn take_ownership_sum(v: Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in v {
        sum += value;
    }
    return sum;
}

fn borrow_ownership_sum(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in v {
        sum += *value;
    }
    return sum;
}

fn cap_values_owned(max: i32, mut v: Vec<i32>) -> Vec<i32> {
    for index in 0..v.len() {
        if v[index] > max {
            v[index] = max;
        }
    }
    return v;
}

fn cap_values_borrow(max: i32, v: &mut Vec<i32>) {
    for index in 0..v.len() {
        if v[index] > max {
            v[index] = max;
        }
    }
}

fn multiple_borrows() {
    let mut values = vec![1, 2, 3, 4, 5];

    let a = &values;
    let b = &values;

    println!("{}, {}", a[2], b[2]);
    /*
    values[2] = 1000; // compile error: mutable borrow occurs here
    println!("{}", values[2]);
    println!("{}, {}", a[2], b[2]);
    */
}

fn main() {
    let values = vec![1, 2, 3, 4, 5];
    let sum = take_ownership_sum(values);
    println!("{}", sum);
//    println!("Sum of {} values: {}", values.len(), sum); // compile error: borrow of moved value: `values`

    let values2 = vec![1, 2, 3, 4, 5];
    let sum2 = borrow_ownership_sum(&values2);
    println!("Sum of {} values: {}", values2.len(), sum2); // this works

    let mut values3 = vec![1, 2, 3, 10000, 5];
    values3 = cap_values_owned(10, values3);
    for v in values3 {
        println!("{}", v);
    }
    println!("====");

    let mut values4 = vec![1, 2, 3, 10000, 5];
    cap_values_borrow(10, &mut values4);
    for v in values4 {
        println!("{}", v);
    }
    println!("====");

    multiple_borrows();
}
