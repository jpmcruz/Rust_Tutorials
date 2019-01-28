fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    x = 5;

    x = x + 1;

    x = x * 2;

    println!("The value of x is: {}", x);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x, y, z is: {}, {}, {}", x, y, z);


    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The values are: {}, {}, {}", five_hundred, six_point_four, one);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The array is : {}", a[0]);

}
