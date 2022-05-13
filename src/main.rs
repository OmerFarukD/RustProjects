#[allow(dead_code)]
#[allow(unused_variables)]
fn if_state(){
    let sicaklik=25;
    if sicaklik>20 {
        println!("Sıcaklık 20 den büyüktür.")
    }
    let gun=if sicaklik>20 {"Günesli"} else { "Bulutlu" };
    println!("Gün : {} ",gun);
}

fn while_and_loop(){
    let mut i=10;
    while i<20 {
        i+=1;
        if i==12 {continue }
        println!("{}",i);


    }

    let mut y=2;
    loop {  //while true gibi çalışır
        y*=2;
        println!("{}",y);
        if  y==1<<10 {break}
    }
}
fn main() {
   // if_state();
    while_and_loop();
}
