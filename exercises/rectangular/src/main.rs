use clap::Parser;


#[derive(Parser)]
#[clap(author, version, about)]
struct Arguments {
    #[clap(short = 'W', long, help = "Width of the rectangle")]
    width: usize,
    #[clap(short = 'H', long, help = "Height of the rectangle")]
    height: usize,
    #[clap(short, long, help = "Use ASCII only characters to draw the rectangle")]
    ascii_only: bool,
    #[clap(short, long, help = "Use thicker lines only to draw the rectangle")]
    bold_lines: bool,
}


fn main() {
    let arguments = Arguments::parse();
    dbg!(arguments.width, arguments.height);
}