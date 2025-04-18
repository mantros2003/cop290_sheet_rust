use std::io::{self, Write};

pub fn get_ip(sz: usize) -> String {
    let mut input = String::new();
    
    // Flush stdout to ensure any prompt is shown before user input
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut input).unwrap();
    
    // Trim newline and clip to maximum allowed length (sz - 1)
    input.trim_end().chars().take(sz - 1).collect()
}
