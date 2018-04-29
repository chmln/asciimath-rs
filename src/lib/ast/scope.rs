use std::collections::HashMap;
pub struct Scope {
    variables: HashMap<String, f64>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            variables: HashMap::new(),
        }
    }

    pub fn set_var<T>(&mut self, var_name: &str, value: T) -> ()
    where
        T: Copy + Into<f64> + PartialOrd + Clone,
    {
        self.variables
            .insert(var_name.to_string(), value.into());
    }

    pub fn get_var(&self, var_name: &str) -> Option<&f64> {
        self.variables.get(var_name)
    }
}
