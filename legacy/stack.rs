// ©2024 - BestOrchid - BestMat - Yuvanth.B - All rights reserved.
#[derive(Debug, Clone)]
struct Stack {
    map: HashMap<String, Variable>,
}

impl Stack {
    fn new() -> Self {
        return Stack {
            map: HashMap::new()
        };
    }
    
    fn push(&mut self, varname: String, value: Variable) {
        self.map.insert(varname, value);
    }
    
    fn set(&mut self, varname: String, value: Type) {
        if let Some(existing_var) = self.map.get(&varname) {
            if existing_var.is_constant {
                panic!("BestOrchid: Cannot assign to constant variable.");
            }
    
            if std::mem::discriminant(&existing_var.val_type) == std::mem::discriminant(&value) {
                match (&existing_var.val_type, &value) {
                    (Type::str(old_str), Type::str(new_str)) => {
                        if old_str.len() != new_str.len() {
                            panic!(
                                "BestOrchid: Variable with original string's bytes are not matching with new value's bytes, i.e. length of {} is not equal to {}.",
                                old_str.len(),
                                new_str.len()
                            );
                        }
                    }
                    _ => {}
                }
                
                self.map.insert(varname, Variable {
                    val_type: value,
                    is_constant: false,
                });
            } else {
                panic!(
                    "BestOrchid: Type {:#?} is not matching with type {:#?}.",
                    existing_var.val_type, value
                );
            }
        } else {
            panic!("BestOrchid: Variable `{}` not found.", varname);
        }
    }
    
    fn delete(&mut self, varname: String) {
        let val = self.get(varname.clone()).clone().val_type;
        
        match val {
            Type::nativefn(_) => {
                panic!("BestOrchid: Cannot delete a native function.");
            }
            
            _ => {
                self.map.remove(&varname);
            }
        }
    }
    
    fn get(&self, varname: String) -> &Variable {        
        if let Some(value) = self.map.get(&varname) {
            return value;
        } else {
            panic!("BestOrchid: Unknown variable or function {}.", varname);
        }
    }
    
    fn call(&self, function: String, args: Vec<Type>) {
        let val = self.get(function).clone().val_type;
        
        match val {
            Type::nativefn(func) => {
                (func.function)(args);
            }
            
            _ => {
                panic!("BestOrchid: Cannot call a non-function having type {:#?}.", val);
            }
        }
    }
    
    fn print(self) {
        println!("{:#?}", self);
    }
}

fn stdlib_println(args: Vec<Type>) -> Result<Type, String> {
    if args.len() < 1 {
        panic!("BestOrchid: Function println has to have atleast 1 argument.");
    } else {
        println!("{:#?}", args[0]);
        
        return Ok(Type::bool(true));
    }
}

/* Tests:
let mut stack = Stack::new();
stack.push(String::from("x"), Variable { val_type: Type::n8(21), is_constant: true });
stack.push(String::from("y"), Variable { val_type: Type::n8(27), is_constant: true });
stack.push(String::from("z"), Variable { val_type: Type::n8(0), is_constant: false });
stack.push(String::from("a"), Variable { val_type: Type::n8(111), is_constant: false });
stack.set(String::from("z"), Type::n8(1));
stack.push(String::from("s"), Variable { val_type: Type::string(OrchidString::new(String::from("Hello"))), is_constant: false });
stack.delete(String::from("a"));

stack.push(String::from("ptr"), Variable { val_type: Type::pointer(OrchidPointer::new(String::from("s"))), is_constant: false });
stack.set(String::from("s"), Type::string(OrchidString::new(String::from("Hi"))));
stack.push(String::from("obj"), Variable { val_type: Type::object(OrchidObject::new()), is_constant: false });

stack.push(String::from("println"), Variable { val_type: Type::nativefn(OrchidNativeFunction { function: stdlib_println }), is_constant: true });
stack.call(String::from("println"), vec![Type::str(String::from("Nagapillaiyar"))]);

// let ptr = stack.get(String::from("ptr")).val_type.clone();
// if let Type::pointer(ref pointer) = ptr {
//     let variable = pointer.get(stack);
//     println!("{:#?}", variable.val_type);
// }

stack.clone().print();
*/