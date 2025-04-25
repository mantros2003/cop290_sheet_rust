#[derive(Default, Debug, PartialEq)]
pub struct Response {
    pub status: i32,
    pub func: i32,
    pub target: i32,
    pub arg1: i32,
    pub arg2: i32,
    pub arg_type: i32,
}

fn ib(integer: i32) -> bool {
    if integer == 0 {
        return false;
    }
    return true;
}

fn valid_row(s: &str) -> bool {
    let len = s.len();

    if len == 0 || len > 3 {
        return false;
    }

    if len > 1 && s.starts_with('0') {
        return false;
    }

    if !s.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    true
}

fn valid_row2(s: &str) -> i32 {
    let len = s.len();

    if len == 0 || len > 3 {
        return 0;
    }

    if len > 1 && s.starts_with('0') {
        return 0;
    }

    if !s.chars().all(|c| c.is_ascii_digit()) {
        return 0;
    }

    s.parse::<i32>().unwrap_or(0)
}

fn valid_column(s: &str) -> bool {
    let len = s.len();

    if len == 0 || len > 3 {
        return false;
    }

    if !s.chars().all(|c| c.is_ascii_uppercase()) {
        return false;
    }

    true
}

fn valid_column2(s: &str) -> i32 {
    let len = s.len();

    if len == 0 || len > 3 {
        return 0;
    }

    let mut ans = 0;
    for c in s.chars() {
        if !c.is_ascii_uppercase() {
            return 0;
        }
        ans = 26 * ans + (c as i32 - 'A' as i32 + 1);
    }

    ans
}

fn valid_integer(s: &str) -> bool {
    let len = s.len();
    if len == 0 {
        return false;
    }

    let bytes = s.as_bytes();

    if bytes[0] == b'-' || bytes[0] == b'+' {
        if len == 1 {
            return false;
        }
        if len == 2 && bytes[1] == b'0' {
            return true;
        } else if bytes[1] == b'0' {
            return false;
        }
        return bytes[1..].iter().all(|&c| c.is_ascii_digit());
    } else {
        if len == 1 && bytes[0] == b'0' {
            return true;
        } else if bytes[0] == b'0' {
            return false;
        }
        return bytes.iter().all(|&c| c.is_ascii_digit());
    }
}

fn valid_cell(s: &str) -> (bool, Option<(String, String)>) {
    let len = s.len();
    if len < 2 || len > 6 {
        return (false, None);
    }

    let bytes = s.as_bytes();
    let mut idx = 0;

    while idx < len {
        if bytes[idx].is_ascii_digit() {
            break;
        }
        idx += 1;
    }

    if idx > 0 && idx < len {
        let col_part = &s[..idx];
        let row_part = &s[idx..];

        if valid_column(col_part) && valid_row(row_part) {
            return (true, Some((col_part.to_string(), row_part.to_string())));
        }
    }

    (false, None)
}

fn valid_cell3(s: &str) -> i32 {
    let len = s.len();
    if len < 2 || len > 6 {
        return 0;
    }

    let mut idx = 0;

    while idx < len {
        if s.as_bytes()[idx].is_ascii_digit() {
            break;
        }
        idx += 1;
    }

    if idx > 0 && idx < len {
        if len - idx > 3 && idx >= 4 {
            return 0;
        }

        let col_part = &s[..idx];
        let row_part = &s[idx..];

        let col = valid_column2(col_part);
        let row = valid_row2(row_part);

        if col > 0 && row > 0 {
            return 1000 * col + row;
        }
    }

    0
}

fn valid_function(s: &str) -> i32 {
    match s {
        "MIN" => 1,
        "MAX" => 2,
        "AVG" => 3,
        "SUM" => 4,
        "STDEV" => 5,
        "SLEEP" => 6,
        _ => 0,
    }
}

fn valid_comp(r1: &str, r2: &str, s1: &str, s2: &str) -> bool {
    if r1.len() > s1.len() {
        return false;
    } else if r1.len() == s1.len() {
        if s1 >= r1 {
            if s2.parse::<i32>().unwrap_or(0) >= r2.parse::<i32>().unwrap_or(0) {
                return true;
            }
        }
    } else {
        if s2.parse::<i32>().unwrap_or(0) >= r2.parse::<i32>().unwrap_or(0) {
            return true;
        }
    }
    return true;
}

fn valid_range(s: &str, equal_expr: &mut String, post_expr: &mut String) -> i32 {
    let length = s.len();

    if length > 4 {
        let mut idx = 0;

        while idx < length {
            if s.as_bytes()[idx] == b':' {
                break;
            }
            idx += 1;
        }

        if idx == length || idx > 6 || idx < 2 {
            return 0;
        }

        equal_expr.clear();
        equal_expr.push_str(&s[..idx]);

        let (is_valid1, cell_parts1) = valid_cell(equal_expr);
        if is_valid1 {
            if let Some((r1, r2)) = cell_parts1 {
                let remaining_length = length - idx - 1;

                if remaining_length > 0 && remaining_length < 7 {
                    post_expr.clear();
                    post_expr.push_str(&s[idx + 1..]);

                    let (is_valid2, cell_parts2) = valid_cell(post_expr);
                    if is_valid2 {
                        if let Some((s1, s2)) = cell_parts2 {
                            if valid_comp(&r1, &r2, &s1, &s2) {
                                return 1;
                            } else {
                                return 2;
                            }
                        }
                    }
                }
            }
        }
    }

    0
}

fn ret_values(c: char) -> i32 {
    match c {
        '+' => 3,
        '-' => 4,
        '*' => 5,
        '/' => 6,
        _ => 0,
    }
}

fn valid_post_expr(s: &str, equal_expr: &mut String, post_expr: &mut String) -> i32 {
    let length = s.len();
    if length > 2 {
        let mut idx = 0;
        if s.as_bytes()[0] == b'-' || s.as_bytes()[0] == b'+' {
            idx = 1;
        }

        while idx < length {
            if s.as_bytes()[idx] == b'*'
                || s.as_bytes()[idx] == b'-'
                || s.as_bytes()[idx] == b'+'
                || s.as_bytes()[idx] == b'/'
            {
                break;
            }
            idx += 1;
        }

        if idx >= length - 1 || idx < 1 {
            return 0;
        }

        equal_expr.clear();
        equal_expr.push_str(&s[..idx]);

        let a = valid_cell3(equal_expr);
        let b = valid_integer(equal_expr);
        let mut sum = 0;

        if ib(a) || b {
            if ib(a) {
                sum += 2;
            }

            let _post_length = length - idx - 1;
            post_expr.clear();
            post_expr.push_str(&s[idx + 1..]);

            let c = valid_cell3(post_expr);
            let d = valid_integer(post_expr);

            if ib(c) || d {
                if ib(c) {
                    sum += 1;
                }
                return 1 + sum;
            }
        }
    }
    0
}

fn parse_func(s: &str, equal_expr: &mut String, exp1: &mut String, exp2: &mut String) -> i32 {
    let length = s.len();
    if length < 4 || s.is_empty() {
        return 0;
    }

    let mut count = 0;

    while count < length {
        if s.as_bytes()[count] == b'(' {
            break;
        }
        count += 1;
    }

    if count == length || s.as_bytes()[length - 1] != b')' {
        return 0;
    }

    if length - count <= 2 {
        return 0;
    }

    let function = &s[..count];
    let func = valid_function(function);
    if func == 0 {
        return 0;
    }

    let range = &s[count + 1..length - 1];
    if func == 6 {
        if valid_integer(range) || ib(valid_cell3(range)) {
            let len1 = function.len();
            equal_expr.clear();
            equal_expr.push_str(&s[..len1]);

            let len2 = range.len();
            let k = len1 + 1;
            exp1.clear();
            exp1.push_str(&s[k..k + len2]);

            return func + 6;
        }
    } else {
        let ans = valid_range(range, exp1, exp2);
        if ans == 1 {
            let len1 = function.len();
            equal_expr.clear();
            equal_expr.push_str(&s[..len1]);
            return func + 6;
        } else if ans == 2 {
            return 1;
        }
    }

    0
}

pub fn parse(s: &str) -> Response {
    let mut returns = Response::default();
    let length = s.len();

    if length == 0 {
        returns.status = 2;
        return returns;
    }

    if length == 1 {
        match s.chars().next() {
            Some('w') => {
                returns.status = 0;
                returns.func = 13;
                return returns;
            }
            Some('d') => {
                returns.status = 0;
                returns.func = 14;
                return returns;
            }
            Some('a') => {
                returns.status = 0;
                returns.func = 15;
                return returns;
            }
            Some('s') => {
                returns.status = 0;
                returns.func = 16;
                return returns;
            }
            Some('q') => {
                returns.status = 0;
                returns.func = 17;
                return returns;
            }
            _ => {}
        }
    }

    if s == "disable_output" {
        returns.status = 0;
        returns.func = 18;
        return returns;
    }

    if s == "enable_output" {
        returns.status = 0;
        returns.func = 19;
        return returns;
    }

    if length > 10 {
        let scroll = &s[0..10];
        if scroll == "scroll_to " {
            let cell = &s[10..];
            if ib(valid_cell3(cell)) {
                returns.status = 0;
                returns.func = 20;
                returns.target = valid_cell3(cell);
                returns.arg_type = 2;
                return returns;
            }
        }
    }

    let mut idx = 0;
    while idx < length {
        if s.as_bytes()[idx] == b'=' {
            break;
        }
        idx += 1;
    }

    if idx > 6 || idx == length || idx < 2 {
        returns.status = 1;
        return returns;
    }

    let equal_expr = &s[0..idx];
    if ib(valid_cell3(equal_expr)) {
        returns.target = valid_cell3(equal_expr);
        let remaining = &s[idx + 1..];
        let mut count = 0;
        let mut operator = '\0';

        if remaining.starts_with('-') || remaining.starts_with('+') {
            count = 1;
        }

        while count < remaining.len() {
            if remaining.as_bytes()[count] == b'*'
                || remaining.as_bytes()[count] == b'-'
                || remaining.as_bytes()[count] == b'+'
                || remaining.as_bytes()[count] == b'/'
                || remaining.as_bytes()[count] == b'('
            {
                operator = remaining.as_bytes()[count] as char;
                break;
            }
            count += 1;
        }

        if count == remaining.len() {
            if valid_integer(remaining) {
                returns.status = 0;
                returns.func = 1;
                returns.arg1 = remaining.parse().unwrap_or(0);
                returns.arg_type = 0;
                return returns;
            }

            if ib(valid_cell3(remaining)) {
                returns.status = 0;
                returns.func = 2;
                returns.arg1 = valid_cell3(remaining);
                returns.arg_type = 2;
                return returns;
            }
        } else if operator == '(' {
            let mut ret1 = String::new();
            let mut ret2 = String::new();
            let mut ret3 = String::new();
            let ret = parse_func(remaining, &mut ret1, &mut ret2, &mut ret3);

            if ret == 12 {
                returns.status = 0;
                returns.func = 12;
                if valid_integer(&ret2) {
                    returns.arg1 = ret2.parse().unwrap_or(0);
                    returns.arg_type = 0;
                }
                if ib(valid_cell3(&ret2)) {
                    returns.arg1 = valid_cell3(&ret2);
                    returns.arg_type = 2;
                }
                return returns;
            } else if ret > 6 && ret < 12 {
                let arg1 = valid_cell3(&ret2);
                let arg2 = valid_cell3(&ret3);
                returns.status = 0;
                returns.func = ret;
                returns.arg1 = arg1;
                returns.arg2 = arg2;
                returns.arg_type = 3;
                return returns;
            } else if ret == 1 {
                returns.status = 2; // Error code
                return returns;
            }
        } else {
            let ret = ret_values(operator);
            let mut ret2 = String::new();
            let mut ret3 = String::new();
            let mut argtp = valid_post_expr(remaining, &mut ret2, &mut ret3);
            if argtp != 0 {
                argtp -= 1;
                let mut arg1 = 0;
                let mut arg2 = 0;

                if argtp == 0 || argtp == 1 {
                    arg1 = ret2.parse().unwrap_or(0);
                } else if argtp == 2 || argtp == 3 {
                    arg1 = valid_cell3(&ret2);
                }

                if argtp == 0 || argtp == 2 {
                    arg2 = ret3.parse().unwrap_or(0);
                } else if argtp == 1 || argtp == 3 {
                    arg2 = valid_cell3(&ret3);
                }

                returns.status = 0;
                returns.func = ret;
                returns.arg1 = arg1;
                returns.arg2 = arg2;
                returns.arg_type = argtp;
                return returns;
            }
        }
    }

    returns.status = 1;
    returns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let str = "A1=MAX(B1:X1)";
        let ret = parse(str);

        println!("{ret:?}");
    }
}
