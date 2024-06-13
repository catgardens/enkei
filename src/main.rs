use log::trace;

fn main() {
    saku_logger::init();
    trace!("logger init");
    println!("Hello, world!");
}
