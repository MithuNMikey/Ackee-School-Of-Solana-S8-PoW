///-------------------------------------------------------------------------------
///
/// This is your calculator implementation task 
/// to practice enums, structs, and methods.
/// 
/// Complete the implementation of the Calculator struct and its methods.
/// 
/// The calculator should support basic arithmetic 
/// operations (addition, subtraction, multiplication)
/// with overflow protection and maintain a history 
/// of operations.
/// 
/// Tasks:
/// 1. Implement the OperationType enum methods
/// 2. Implement the Operation struct constructor
/// 3. Implement all Calculator methods
/// 
///-------------------------------------------------------------------------------

#[derive(Clone)]
pub enum OperationType {
    Addition,
    Subtraction,
    Multiplication
}

impl OperationType {
    pub fn get_sign(&self) -> &str {
        match self {
            OperationType::Addition => "+",
            OperationType::Subtraction => "-",
            OperationType::Multiplication => "*",
        }
    }
    
    pub fn perform(&self, x: i64, y: i64) -> Option<i64> {
        match self {
            OperationType::Addition => x.checked_add(y),
            OperationType::Subtraction => x.checked_sub(y),
            OperationType::Multiplication => x.checked_mul(y),
        }
    }
}

#[derive(Clone)]
pub struct Operation {
    pub first_num: i64,
    pub second_num: i64,
    pub operation_type: OperationType
}

impl Operation {
    pub fn new(first_num: i64, second_num: i64, operation_type: OperationType) -> Self {
        Operation {
            first_num,
            second_num,
            operation_type,
        }
    }
}

pub struct Calculator {
    pub history: Vec<Operation>
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            history: Vec::new(),
        }
    }
    
    pub fn addition(&mut self, x: i64, y: i64) -> Option<i64> {
        let result = OperationType::Addition.perform(x, y)?;
        self.history.push(Operation::new(x, y, OperationType::Addition));
        Some(result)
    }
    
    pub fn subtraction(&mut self, x: i64, y: i64) -> Option<i64> {
        let result = OperationType::Subtraction.perform(x, y)?;
        self.history.push(Operation::new(x, y, OperationType::Subtraction));
        Some(result)
    }
    
    pub fn multiplication(&mut self, x: i64, y: i64) -> Option<i64> {
        let result = OperationType::Multiplication.perform(x, y)?;
        self.history.push(Operation::new(x, y, OperationType::Multiplication));
        Some(result)
    }
    
    pub fn show_history(&self) -> String {
        let mut history_str = String::new();
        for (index, operation) in self.history.iter().enumerate() {
            let result = operation.operation_type.perform(operation.first_num, operation.second_num).unwrap();
            history_str.push_str(&format!(
                "{}: {} {} {} = {}\n",
                index,
                operation.first_num,
                operation.operation_type.get_sign(),
                operation.second_num,
                result
            ));
        }
        history_str
    }
    
    pub fn repeat(&mut self, operation_index: usize) -> Option<i64> {
        if operation_index >= self.history.len() {
            return None;
        }
        
        let operation = self.history[operation_index].clone();
        let result = operation.operation_type.perform(operation.first_num, operation.second_num)?;
        
        self.history.push(operation);
        Some(result)
    }
    
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}