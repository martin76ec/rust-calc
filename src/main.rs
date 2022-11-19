use regex::Regex;

static RE_ADD: &str = r"(\d+)\s?\+\s?(\d+)";
static RE_DIV: &str = r"(\d+)\s?/\s?(\d+)";
static RE_MUL: &str = r"(\d+)\s?\*\s?(\d+)";
static RE_SUB: &str = r"(\d+)\s?\-\s?(\d+)";
static ADD: fn(l_val: i32, r_val: i32) -> i32 = |l_val: i32, r_val: i32| -> i32 { l_val + r_val };
static DIV: fn(l_val: i32, r_val: i32) -> i32 = |l_val: i32, r_val: i32| -> i32 { l_val / r_val };
static MUL: fn(l_val: i32, r_val: i32) -> i32 = |l_val: i32, r_val: i32| -> i32 { l_val * r_val };
static SUB: fn(l_val: i32, r_val: i32) -> i32 = |l_val: i32, r_val: i32| -> i32 { l_val - r_val };

fn make_operation(rgx: String, op: fn(l_val: i32, r_val: i32) -> i32) -> impl Fn(String) -> String {
    return move |expr: String| -> String {
        let mut expression = expr;

        loop {
            let caps = Regex::new(rgx.as_str()).unwrap().captures(expression.as_str());
            if caps.is_none() { break };

            let caps = caps.unwrap();
            let cap_expression = caps.get(0).unwrap().as_str();
            let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
            let op_result = op(left_value, right_value);

            expression = expression.replace(cap_expression, &op_result.to_string());
        }

        return expression;
    };
}

fn main() {
    let add = make_operation(RE_ADD.to_string(), ADD);
    let divide = make_operation(RE_DIV.to_string(), DIV);
    let multiply = make_operation(RE_MUL.to_string(), MUL);
    let substract = make_operation(RE_SUB.to_string(), SUB);

    println!("Please, add your expression:");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    expression = multiply(expression);
    expression = divide(expression);
    expression = add(expression);
    expression = substract(expression);

    println!("Result: {}", expression);
}