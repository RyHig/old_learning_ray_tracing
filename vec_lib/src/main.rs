use vec_lib::Vector3;

fn main() {
    let nx: i64 = 200;
    let ny: i64 = 100;
    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut a = Vector3((i as f64)/(nx as f64), 
                                (j as f64)/(ny as f64),
                                0.2);
            a = a * 255.99;
            println!("{} {} {}\n", a.0 as i64, a.1 as i64, a.2 as i64);
        }
    }

    /*let a = Vector3 {
        e0: 5.0,
        e1: 4.0,
        e2: 3.0};
    let b = Vector3 {
        e0: 4.0,
        e1: 5.0,
        e2: 6.0};
    let c = a + b;
    println!("{:?}", c);*/
}
