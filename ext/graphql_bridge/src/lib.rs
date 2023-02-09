use magnus::{
    RModule,
    define_module,
    method,
    prelude::*,
    Error, RString,
};
use apollo_parser::{Parser};

fn parse(_: RModule, input: RString) -> Result<bool, Error> {
    // RString::as_str is unsafe as it's possible for Ruby to invalidate the
    // str as we hold a reference to it.
    let input = unsafe { input.as_str()? };
    let parser = Parser::new(input);
    let ast = parser.parse();
    println!("{:?}", ast);

    Ok(true)
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("GraphQLBridge")?;
    module.define_singleton_method("parse", method!(parse, 1))?;
    Ok(())
}
