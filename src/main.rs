use evie::Evie;

pub mod evie;

//Just an entry point.
fn main() {
    let mut beep_boop: Evie = Evie{

    };

    beep_boop.init();
    beep_boop.run();
}
