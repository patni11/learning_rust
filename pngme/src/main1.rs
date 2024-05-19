// mod args;
// mod chunk;
mod chunk_type;
// mod commands;
// mod png;

pub type Error = Box<dyn std::error::Error>; //creating a custom error type that uses dynamic dispatch 
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    todo!()
}
