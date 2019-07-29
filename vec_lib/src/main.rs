use vec_lib::Vector3;

fn main() {
    let nx: i64 = 200;
    let ny: i64 = 100;
    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut a = Vector3::new((i as f64)/(nx as f64), 
                                (j as f64)/(ny as f64),
                                0.5);
            a = a * 255.0;

            println!("{} {} {}\n", a.r() as i64, a.g() as i64, a.b() as i64);
        }
    }
}
