use std::collections::HashMap;

// https://www.codewars.com/kata/58e24788e24ddee28e000053

// either variable name or a value
enum Operand {
    Value(i64),
    Variable(String),
}
impl Operand {
    fn from_string(s: &str) -> Self {
        if let Ok(numb) = s.parse::<i64>() {
            Operand::Value(numb)
        }
        else {
            Operand::Variable(s.to_string())
        }
    }
}

enum Instruction {
    Mov(String, Operand),
    Inc(String),
    Dec(String),
    Jnz(Operand, Operand),
}

impl Instruction {
    fn from_string(s: &str) -> Self {
        let exp: Vec<_> = s.split_whitespace().collect();
        if exp.len() < 2 {
            panic!("parse error");
        }
        else {
            match exp[0] {
                "mov" => Instruction::Mov(exp[1].to_string(), Operand::from_string(exp[2])),
                "inc" => Instruction::Inc(exp[1].to_string()),
                "dec" => Instruction::Dec(exp[1].to_string()),
                "jnz" => Instruction::Jnz(Operand::from_string(exp[1]), Operand::from_string(exp[2])),
                _ => panic!("parse error!")
            }
        }
    }
}

pub struct Program {
    pub registry: HashMap<String, i64>,
    instructions: Vec<Instruction>
}

impl Program {
    pub fn new(instructions: Vec<&str>) -> Self {
        Self {
            registry: Default::default(),
            instructions: instructions.iter().map(|&s| Instruction::from_string(s)).collect()
        }
    }

    fn resolve_value(&self, op: &Operand) -> i64 {
        match op {
            Operand::Value(n) => *n,
            Operand::Variable(var_name) => *self.registry.get(var_name).unwrap(),
        }
    }

    pub fn run(&mut self) {
        // clear previous program state
        self.registry.clear();

        let mut instruction_index = 0;

        while instruction_index < self.instructions.len() {
            match &self.instructions[instruction_index] {
                Instruction::Mov(variable, value) => {
                    let val = self.resolve_value(value);
                    // upsert
                    *self.registry.entry(variable.clone()).or_default() = val;
                    instruction_index += 1;
                },
                Instruction::Inc(variable) => { // should always have a prior mov
                    self.registry.entry(variable.clone()).and_modify(|x| { *x +=1; });
                    instruction_index += 1;
                },
                Instruction::Dec(variable)  => { // should always have a prior mov
                    self.registry.entry(variable.clone()).and_modify(|x| { *x -=1; });
                    instruction_index += 1;
                },
                Instruction::Jnz(variable, jump)  => {
                    let val = self.resolve_value(variable);
                    // (ignore jump here to move to next instruction)
                    if val == 0 {
                        instruction_index += 1;
                    }
                    else {
                        let val = self.resolve_value(jump);
                        if val < 0 {
                            instruction_index -= (val * -1) as usize;
                        }
                        else {
                            instruction_index += val as usize;
                        }
                    }
                },
            }
        }
    }
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! map {
        ($($key:expr => $value:expr),*) => {{
             let mut map = HashMap::new();
             $(
                 map.insert($key.to_string(), $value);
             )*
             map
        }};
    }

    #[test]
    fn short_tests() {
        let program = vec!["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"];
        let expected = map! { "a" => 1 };
        let mut program = Program::new(program);
        program.run();
        compare_registers(expected, program.registry);

        let program = vec![
            "mov c 12",
            "mov b 0",
            "mov a 200",
            "dec a",
            "inc b",
            "jnz a -2",
            "dec c",
            "mov a b",
            "jnz c -5",
            "jnz 0 1",
            "mov c a",
        ];
        let mut program = Program::new(program);
        program.run();
        let expected = map! { "a" => 409600, "c" => 409600, "b" => 409600};
        compare_registers(expected, program.registry);
    }

    fn compare_registers(expected: HashMap<String, i64>, actual: HashMap<String, i64>) {
        let result = expected
            .iter()
            .all(|(key, value)| actual.get(key).map(|v| v == value).unwrap_or(false));
        assert!(
            result,
            "Expected the registers to be like that:\n{:#?}\n\nBut got this:\n{:#?}\n",
            expected, actual
        )
    }
}