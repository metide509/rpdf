mod header;
mod trailer;
mod pdf;

use pdf::PDF;

fn main() {
    let file = "C:\\Users\\metide\\Desktop\\１６542大阳四轮车.pdf";
    let pdf = PDF::from(file);
}



