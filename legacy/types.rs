// ©2024 - BestOrchid - BestMat - Yuvanth.B - All rights reserved.
#[derive(Debug, PartialEq, Clone)]
enum Type {
    n8(i8), n16(i16), n32(i32), n64(i64), n128(i128), // Numbers (Integers)
    u8(u8), u16(u16), u32(u32), u64(u64), u128(u128), // Unsigned Numbers
    f32(f32), f64(f64),                               // Floats
    str(String),                                      // Constant String
    bool(bool),                                       // Boolean
    string(OrchidString),                             // Dynamic String
    array(OrchidArray),                               // Array (Static)
    vector(OrchidVector),                             // Vector (Dynamic)
    pointer(OrchidPointer),                           // Pointer
    object(OrchidObject),                             // Object
    nativefn(OrchidNativeFunction),                   // Native Functions
    null(i8),                                         // Null
}

#[derive(Debug, PartialEq, Clone)]
struct Variable {
    val_type: Type,
    is_constant: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct OrchidString {
    value: Box<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct OrchidArray {
    value: Vec<Type>,
    length: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct OrchidVector {
    value: Vec<Type>,
    length: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct OrchidPointer {
    pointer: String,
}

#[derive(Debug, Clone, PartialEq)]
struct OrchidObject {
    value: HashMap<String, Type>,
    keys: OrchidVector,
    values: OrchidVector,
    length: usize
}

#[derive(Debug, Clone, PartialEq)]
struct OrchidNativeFunction {
    function: fn(Vec<Type>) -> Result<Type, String>,
}

impl OrchidString {
    fn new(value: String) -> Self {
        return OrchidString {
            value: Box::new(value)
        };
    }
    
    fn len(&self) -> usize {
        return self.value.len();
    }
    
    fn set(&mut self, new_val: String) {
        self.value = Box::new(new_val);
    }
    
    fn to_rust_string(self) -> String {
        return *self.value;
    }
    
    fn as_str(&self) -> &str {
        return &self.value;
    }
}

impl OrchidArray {
    fn new(array: Vec<Type>) -> Self {
        return OrchidArray {
            length: array.len(),
            value: array,
        };
    }
    
    fn len(&self) -> usize {
        return self.length;
    }
    
    fn get(&self) -> Vec<Type> {
        return self.value.clone();
    }
    
    fn includes(&self, search_item: &Type) -> bool {
        return self.value.clone().iter().any(|item| item == search_item);
    }
    
    fn at(&self, search_item: &Type) -> Option<usize> {
        return self.value.iter().position(|item| item == search_item);
    }
    
    fn get_index(&self, index: usize) -> Type {
        if index < self.len() {
            return self.value[index].clone();
        } else {
            panic!("BestOrchid: Unsafe Code - Cannot access index {} when length of the array is {}.", index, self.len());
        }
    }
}

impl OrchidVector {
    fn new(array: Vec<Type>) -> Self {
        return OrchidVector {
            length: array.len(),
            value: array,
        };
    }
    
    fn len(&self) -> usize {
        return self.length;
    }
    
    fn get(&self) -> Vec<Type> {
        return self.value.clone();
    }
    
    fn includes(&self, search_item: &Type) -> bool {
        return self.value.clone().iter().any(|item| item == search_item);
    }
    
    fn at(&self, search_item: &Type) -> Option<usize> {
        return self.value.iter().position(|item| item == search_item);
    }
    
    fn get_index(&self, index: usize) -> Type {
        if index < self.len() {
            return self.value[index].clone();
        } else {
            panic!("BestOrchid: Unsafe Code - Cannot access index {} when length of the vector is {}.", index, self.len());
        }
    }
    
    fn push(&mut self, value: Type) {
        self.value.push(value);
    }
    
    fn pop(&mut self) {
        self.value.pop();
    }
}

impl OrchidPointer {
    fn new(pointer: String) -> Self {
        return OrchidPointer {
            pointer,
        };
    }
    
    fn get(&self, stack: Stack) -> Variable {
        if let Some(var) = stack.map.get(&self.pointer) {
            return var.clone();
        } else {
            panic!("BestOrchid: The pointer points to an unknown variable {}", self.pointer);
        }
    }
    
    fn set(&self, mut stack: Stack, value: Type) {
        stack.set(self.pointer.clone(), value);
    }
}

impl OrchidObject {
    fn new() -> Self {
        return OrchidObject {
            value: HashMap::new(),
            keys: OrchidVector::new(Vec::new()),
            values: OrchidVector::new(Vec::new()),
            length: 0
        };
    }
    
    fn set(&mut self, key: String, value: Type) {
        self.value.insert(key.clone(), value.clone());
        self.keys.push(Type::str(key));
        self.values.push(value);
        self.length = self.length + 1;
    }
    
    fn get(&self, key: String) -> Type {
        if let Some(value) = self.value.get(&key) {
            return value.clone();
        } else {
            panic!("BestOrchid: Unknown key in object - {}", key);
        }
    }
    
    fn keys(&self) -> OrchidArray {
        return OrchidArray {
            value: self.keys.get(),
            length: self.len()
        };
    }
    
    fn values(&self) -> OrchidArray {
        return OrchidArray {
            value: self.values.get(),
            length: self.len()
        };
    }
    
    fn len(&self) -> usize {
        return self.length;
    }
}