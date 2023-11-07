use evie::Evie;

pub mod evie;

use tch::Tensor;

//Just an entry point.
fn main() {

    let t = Tensor::from_slice(&[3, 1, 4, 1, 5]);
    let t = t * 2;
    t.print();
    
    let mut beep_boop: Evie = Evie{

    };
    //hellloooooo

    beep_boop.init();
    beep_boop.run();
}
