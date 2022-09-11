pub fn show_help() {
    println!("input_file.extension output_file.extension -operation {{extra parameters}}");

    println!("\tResize: -rs {{width}} {{height}}");
    println!("\tGrayscale: -gs");
    println!("\tSingle out colour: -soc {{colour (-red / -green / -blue)}}");
    println!("\tConvert to ASCII: -ctac");
    println!("\tBlur: -blur");

    println!();

    std::process::exit(0);
}