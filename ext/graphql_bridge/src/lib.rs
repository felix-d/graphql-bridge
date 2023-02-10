use magnus::{
    method,
    Value,
    prelude::*,
    Error, RString, define_class, function,
};
use apollo_parser::{Parser, SyntaxTree, ast::{Document, Definition, OperationDefinition, SelectionSet, Selection, Field}};
use convert_case::{Case, Casing};

#[magnus::wrap(class = "GraphQLBridge")]
struct GraphQLBridge {
    schema: Value,
}

impl GraphQLBridge {
    fn new(schema: Value) -> Self {
        Self {
            schema,
        }
    }

    fn eval(&self, input: RString) -> Result<bool, Error> {
        let ast = self.parse_input(input)?;
        self.eval_root(ast.document());
        Ok(true)
    }

    fn eval_root(&self, root: Document) {
        for def in root.definitions() {
            if let Definition::OperationDefinition(op_def) = def {
                self.eval_definition(op_def);
            }
        }
    }

    fn eval_definition(&self, op_def: OperationDefinition) {
        if let Some(set) = op_def.selection_set() {
            self.eval_selection_set(set);
        }
    }

    fn eval_selection_set(&self, selection_set: SelectionSet) {
        for selection in selection_set.selections() {
            self.eval_selection(selection);
        }
    }

    fn eval_selection(&self, selection: Selection) {
        match selection {
            Selection::Field(field) => {
                self.eval_field(field);
            },
            _ => {}
        }
    }

    fn eval_field(&self, field: Field) {
        if let Some(name) = field.name() {
            let text = name.text().as_str().to_case(Case::Snake);
            let _value: Result<Value, magnus::Error> = self.schema.funcall(text, ());
        }
    }

    fn parse_input(&self, input: RString) -> Result<SyntaxTree, Error> {
        let input = unsafe { input.as_str()? };
        let parser = Parser::new(input);
        let ast = parser.parse();
        Ok(ast)
    }
}


#[magnus::init]
fn init() -> Result<(), Error> {
    let class = define_class("GraphQLBridge", Default::default())?;
    class.define_singleton_method("new", function!(GraphQLBridge::new, 1))?;
    class.define_method("eval", method!(GraphQLBridge::eval, 1))?;
    Ok(())
}
