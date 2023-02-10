use magnus::{
    method,
    class,
    Value,
    prelude::*,
    Error, RString, define_class, function,
};
use apollo_parser::{Parser, ast::{Definition, OperationDefinition, SelectionSet, Selection, Field}};
use convert_case::{Case, Casing};
use serde_json::{json};

type JsonOutput = serde_json::Value;

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

    // Parses a GraphQL query injected from Ruby land and evaluates the
    // resulting AST by calling into a Ruby schema object.
    fn eval(&self, input: RString) -> Result<String, Error> {
        let query = unsafe { input.as_str()? };
        let parser = Parser::new(query);
        let ast = parser.parse();
        let mut result = json!({});

        for def in ast.document().definitions() {
            if let Definition::OperationDefinition(op_def) = def {
                self.eval_definition(op_def, self.schema, &mut result);
            }
        }

        Ok(result.to_string())
    }

    fn eval_definition(&self, op_def: OperationDefinition, graph_model: Value, output: &mut JsonOutput) {
        if let Some(set) = op_def.selection_set() {
            self.eval_selection_set(set, graph_model, output);
        }
    }

    fn eval_selection_set(&self, selection_set: SelectionSet, graph_model: Value, output: &mut JsonOutput) {
        for selection in selection_set.selections() {
            self.eval_selection(selection, graph_model, output);
        }
    }

    fn eval_selection(&self, selection: Selection, graph_model: Value, output: &mut JsonOutput) {
        match selection {
            Selection::Field(field) => {
                self.eval_field(field, graph_model, output);
            },
            _ => {}
        }
    }

    fn eval_field(&self, field: Field, graph_model: Value, output: &mut JsonOutput) {
        if let Some(name) = field.name() {
            let name = name.text();
            let value: Value = graph_model.funcall(name.to_case(Case::Snake), ()).unwrap();

            if value.is_kind_of(class::string()) {
                output[name.as_str()] = serde_json::Value::String(value.to_string());
            } else {
                output[name.as_str()] = json!({});

                self.eval_selection_set(
                    field.selection_set().unwrap(),
                    value,
                    &mut output[name.as_str()],
                );
            }
        }
    }
}


#[magnus::init]
fn init() -> Result<(), Error> {
    let class = define_class("GraphQLBridge", Default::default())?;
    class.define_singleton_method("new", function!(GraphQLBridge::new, 1))?;
    class.define_method("eval", method!(GraphQLBridge::eval, 1))?;
    Ok(())
}
