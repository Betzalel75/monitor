

pub fn get_cpu_model(buffer: &str)-> String{
    let mut cpu_model = String::new();
    let mut lines = buffer.lines();
    while let Some(line) = lines.next() {
        if line.starts_with("model name") {
            cpu_model = line.split(": ").nth(1).unwrap_or_default().trim().to_string();
            break;
        }
    }
    cpu_model
}

pub fn parse_memory_line(line: &str) -> u64 {
    line.split_whitespace()
        .nth(1)
        .and_then(|n| n.parse::<u64>().ok())
        .unwrap_or(0)
}