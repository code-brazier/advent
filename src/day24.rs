use std::collections::{HashMap};
use std::fs;
use rand::Rng;

fn main() {
    let wires = &fs::read_to_string("day24a.txt").expect("Unable to read input file");
    let gates = &fs::read_to_string("day24b.txt").expect("Unable to read input file");
    println!("part1: {}", part1(gates, get_wire_values(wires)));
    println!("part2: ");
    let gates = &fs::read_to_string("day24c.txt").expect("Unable to read input file");
    part2(gates);
}

struct Gate {
    input1_label: String,
    input2_label: String,
    input1_value: i32,
    input2_value: i32,
    operation: String,
    output: String
}

impl Gate {
    fn parse(input: &str) -> Gate {
        let tokens = input.split(" ").collect::<Vec<&str>>();
        Gate {
            input1_label: tokens[0].to_string(),
            input2_label: tokens[2].to_string(),
            input1_value: -1,
            input2_value: -1,
            operation: tokens[1].to_string(),
            output: tokens[4].to_string()
        }
    }
}

fn get_gates(gates: &String) -> Vec<Gate> {
    gates.lines().map(|line| Gate::parse(line)).collect()
}

fn part2(gates: &String) {
    test_outputs(gates);
}

fn part1(gates: &String, mut inputs: HashMap<String, i32>) -> String {
    let mut system = get_gates(gates);

    let mut undecided = true;
    while undecided {
        undecided = false;
        for gate in system.iter_mut() {
            let input1_label = &gate.input1_label;
            let input2_label = &gate.input2_label;

            if inputs.contains_key(input1_label) {
                gate.input1_value = inputs[input1_label];
            }
            if inputs.contains_key(input2_label) {
                gate.input2_value = inputs[input2_label];
            }
            if gate.input1_value != -1 && gate.input2_value != -1 {
                inputs.insert(gate.output.clone(), match gate.operation.as_str() {
                    "XOR" => gate.input1_value ^ gate.input2_value,
                    "OR" => gate.input1_value | gate.input2_value,
                    "AND" => gate.input1_value & gate.input2_value,
                    _ => unreachable!()
                });
            } else {
                undecided = true;
            }
        }
    }

    read_outputs(inputs)
}

fn read_outputs(wires: HashMap<String, i32>) -> String {
    let mut i = 0;
    let mut output = String::new();
    loop {
        let wire = format!("z{:02}", i);
        let wire = wire.as_str();
        if !wires.contains_key(wire) {
            break
        }
        output = format!("{}{}", wires[wire], output);
        i += 1;
    }
    i64::from_str_radix(&output, 2).unwrap().to_string()
}

fn get_wire_values(wires: &String) -> HashMap<String, i32> {
    let mut wire_values = HashMap::new();
    for wire_line in wires.lines() {
        let (wire_label, value) = wire_line.split_once(": ").unwrap();
        wire_values.insert(wire_label.to_string(), value.parse::<i32>().unwrap());
    }
    wire_values
}

fn test_outputs(gates: &String) {
    let mut x = String::new();
    let mut y = String::new();
    let mut inputs = HashMap::new();
    for input_index in 0..45 {
        let x_bit = rand::rng().random_range(0..2);
        let y_bit = rand::rng().random_range(0..2);
        x = format!("{}{}", x_bit, x);
        y = format!("{}{}", y_bit, y);
        inputs.insert(format!("x{:02}", input_index), x_bit);
        inputs.insert(format!("y{:02}", input_index), y_bit);
    }

    let output = part1(gates, inputs).parse::<i64>().unwrap();
    let expected_output = i64::from_str_radix(&x, 2).unwrap() + i64::from_str_radix(&y, 2).unwrap();
    if output == expected_output {
        println!("Correct");
        return
    }

    println!("{} + {}: Expected {} was {}", x, y, expected_output, output);

    let mut gates_by_output = HashMap::new();
    let system = get_gates(gates);
    system.iter().for_each(|gate| { gates_by_output.insert(gate.output.clone(), gate); });

    for i in 0..45 {
        let wire = format!("z{:02}", i);
        let wire = wire.as_str();
        if !gates_by_output.contains_key(wire) {
            break
        }
        let output_computation = describe_gate(wire, &gates_by_output, 0);
        println!("{} = {}", wire, output_computation);
        let output_char = 2i64.pow(i) & output;
        let expected_char = 2i64.pow(i) & expected_output;
        if output_char != expected_char {
            println!("Expected {} was {}", expected_char, output_char);
        }
    }
}

fn describe_gate(wire: &str, gates: &HashMap<String, &Gate>, tab: usize) -> String {
    if wire.starts_with("x") || wire.starts_with("y") {
        return wire.to_string()
    }
    let gate = gates[wire];
    format!("({} {} {})", describe_gate(gate.input1_label.as_str(), gates, tab + 1), gate.operation, describe_gate(gate.input2_label.as_str(), gates, tab + 1))
}

#[cfg(test)]
mod tests {
    use crate::{get_wire_values, part1};

    #[test]
    fn test_part1_small() {
        let wires = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0"#;
        let gates = r#"x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;
        assert_eq!(part1(&gates.to_string(), get_wire_values(&wires.to_string())), "4");
    }

    #[test]
    fn test_part1() {
        let wires = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1"#;
        let gates = r#"ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;
        assert_eq!(part1(&gates.to_string(), get_wire_values(&wires.to_string())), "2024");
    }
}
