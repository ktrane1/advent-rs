use clap::{Args, Parser, Subcommand};
use std::fs;

mod day1;
mod day2;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
//#IMPORTMARKER

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "advent")]
#[command(about = "CLI for advent of  code", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// executes current day
    Exec(Exec),
    /// Initializes day with files and subdirectory
    Init(Init),
}

#[derive(Debug, Args)]
struct Exec {
    /// The string to reverse
    day: Option<u16>,
    // flag to use testdata instead of real data. defaults to using real data
    #[clap(default_value_t = false, short, long)]
    test: bool,
}

#[derive(Debug, Args)]
struct Init {
    /// Creates module, and updates main file with import + match statement
    ///
    day: Option<u16>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Exec(exec) => {
            let data_file_path = match exec.test {
                true => format!("./src/day{}/testdata", exec.day.unwrap()),
                false => format!("./src/day{}/data", exec.day.unwrap()),
            };

            let d: Vec<String> = fs::read_to_string(data_file_path)
                .expect("could not read file")
                .lines()
                .map(|s| s.to_string())
                .collect();

            match exec.day {
                Some(1) => day1::day1::execute(d),
                Some(2) => day2::day2::execute(d),
                Some(3) => day3::day3::execute(d),
                Some(4) => day4::day4::execute(d),
                Some(5) => day5::day5::execute(d),
                Some(25) => day25::day25::execute(d),
                Some(6) => day6::day6::execute(d),
                //#CASEMARKER
                None => todo!(),
                _ => todo!(),
            }
        }
        Commands::Init(init) => {
            match init.day {
                Some(day) => {
                    let _res = match fs::read_dir(format!("./src/day{}", day)) {
                        Ok(_) => {
                            println!("Directory {} already exists dummy", day);
                            return ();
                        }
                        Err(_) => {
                            let _dir = fs::create_dir(format!("./src/day{}", day));
                            let _d = fs::File::create(format!("./src/day{}/data", day));
                            let _test_d = fs::File::create(format!("./src/day{}/testdata", day));
                            // create day{DAY}.rs
                            let _ = fs::write(
                                format!("./src/day{}/day{}.rs", day, day),
                                "pub fn execute(data: Vec<String>) {\n   dbg!(\"data\");\n}",
                            );
                            // create mod.rs
                            let _ = fs::write(
                                format!("./src/day{}/mod.rs", day),
                                format!("pub mod day{};", day),
                            );

                            // backup current main.rs
                            let current_main = fs::read_to_string("./src/main.rs").unwrap();
                            let _backup = fs::write("./main_backup.rs", current_main.clone());

                            // new main.rs
                            let add_case = current_main.replace("//#CASEMARKER\n", format!("Some({d}) => day{d}::day{d}::execute(d),\n                //#CASEMARKER\n", d=day).as_str());
                            let add_import = add_case.replace(
                                "//#IMPORTMARKER\n",
                                format!("mod day{};\n//#IMPORTMARKER\n", day).as_str(),
                            );
                            //write new main.rs
                            let _new_main = fs::write("./src/main.rs", add_import);
                            println!("created the files");
                        }
                    };
                }
                None => {
                    println!("Need to provide a day for init subcommand dummy");
                }
            }
        }
    }
}
