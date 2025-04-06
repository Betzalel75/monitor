

pub fn format_memory_size(size: f32) -> String{
    let mut formatted_size = String::new();
    let mut size = size;
    let mut unit = "KB";

    if size >= 1024.0 {
        size /= 1024.0;
        unit = "MB";
    }
    if size >= 1024.0 {
        size /= 1024.0;
        unit = "GB";
    }
    if size >= 1024.0 {
        size /= 1024.0;
        unit = "TB";
    }

    formatted_size.push_str(&format!("{:.2}", size));
    formatted_size.push(' ');
    formatted_size.push_str(unit);

    formatted_size
}

pub fn convert_memory_size(size: f32) -> f32{
    let mut size = size;

    if size >= 1024.0 {
        size /= 1024.0;
    }
    if size >= 1024.0 {
        size /= 1024.0;
    }
    if size >= 1024.0 {
        size /= 1024.0;
    }

    size
}