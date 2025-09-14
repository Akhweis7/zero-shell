pub fn tnanm() {
    const GREEN_NAMES: &str = include_str!("../../green names.txt");
    print!("\x1b[38;5;40m{}\x1b[0m", GREEN_NAMES); // ansi color. \x1b[0m to reset the color
}

pub fn z_shell() {
    const zshell: &str = include_str!("../../0-shell.txt");
    print!("\x1b[38;5;40m{}\x1b[0m", zshell); // ansi color. \x1b[0m to reset the color
}
