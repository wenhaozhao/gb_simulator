use proc_macros::U16FieldAccessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}

#[derive(U16FieldAccessor)]
struct AAA{
    ff: u16
}