#[allow(dead_code)]
#[allow(unused_variables)]
use std::mem;

fn main() {
aritmetikIslemler()
}
fn aritmetikIslemler(){
    let a=2;
    let b=257;
    //let a_kupu=i32::pow(a,3);
    println!("{} nin 3. kuvveti= {}",a,i32::pow(a,3));
println!("{} nın {} ile bölümünden kalan {}",b,a,(b%a));
    let c=2.86;
    println!("{} nin 3. üssü= {}",c,f64::powi(c,3)); //integer yazmamızın sebebi integer üs almamız.
    println!("{} nin PI üssü={}",c,f64::powf(c,std::f64::consts::PI));
    
}
