use slint::SharedString;

slint::include_modules!();

fn transform_digits_to_dots_in(input_str: &str) -> String {
    let mut output = String::new();
    for c in input_str.chars() {
        if let Some(digit) = c.to_digit(10) {
            output.push_str(&".".repeat(digit as usize));
        } else {
            output.push(c);
        }
    }

    output
}

pub fn get_piece_from_fen(fen_str: SharedString, cell_index: i32) -> SharedString {
    let board_part = fen_str.split(" ").nth(0).unwrap();
    let lines = board_part.split("/");
    let lines: Vec<_> = lines.map(|l| transform_digits_to_dots_in(l)).collect();
    let line_value = &lines[cell_index as usize / 8];
    let result = line_value.chars().nth((cell_index % 8) as usize).unwrap();
    let mut output = SharedString::new();
    output.push_str(format!("{}", result).as_str());

    output
}

pub fn is_white_turn_in_fen(fen_str: SharedString) -> bool {
    let turn_part = fen_str.split(" ").nth(1).unwrap();
    turn_part == "w"
}