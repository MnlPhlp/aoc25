use aoc23::{run_parallel, run_serial, Task, DAY_COUNT};
use clap::Parser;
use std::{
    fmt::Write,
    time::{Duration, Instant},
    vec,
};

#[cfg(test)]
mod test;

fn cap_length(msg: &str, length: usize) -> &str {
    if msg.len() <= length {
        return msg;
    }
    &msg[0..length]
}

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value = "false")]
    test: bool,
    #[clap(short, long, default_value = "0")]
    day: usize,
    #[clap(long, default_value = "0")]
    task: u8,
    #[clap(short, long, default_value = "false")]
    parallel: bool,
    #[clap(long)]
    /// used for vscode debug launch config
    day_string: Option<String>,
}

fn main() -> std::fmt::Result {
    // parse command line arguments
    let args: Args = Args::parse();
    let test = args.test;
    let task = if args.task == 1 {
        Task::One
    } else if args.task == 2 {
        Task::Two
    } else {
        Task::Both
    };
    let parallel = args.parallel;

    let day = match args.day_string {
        Some(day) => day.replace("day", "").parse::<usize>().unwrap_or(args.day),
        None => args.day,
    };
    let days = if day == 0 {
        (1..=DAY_COUNT).collect::<Vec<usize>>()
    } else {
        vec![day]
    };
    println!("Calculating days: {days:?}");

    let mut results1 = vec![String::new(); days.len()];
    let mut results2 = vec![String::new(); days.len()];
    let mut times = vec![Duration::new(0, 0); days.len()];

    let start = Instant::now();
    if parallel {
        run_parallel(&days, &mut results1, &mut results2, &mut times, test, task);
    } else {
        run_serial(&days, &mut results1, &mut results2, &mut times, test, task);
    }
    let overall = Instant::now().duration_since(start);

    let mut results: String = "## Results:\n".into();
    results += "day | result 1        | result 2        | time      | % overall \n";
    results += "--: | :-------------: | :--------------:| --------: | :--------\n";
    for (i, day) in days.iter().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        writeln!(
            results,
            "{: >3} | {: <15} | {: <15} | {: >9.2?} | {: >4.2} %",
            day,
            cap_length(&results1[i], 15),
            cap_length(&results2[i], 15),
            times[i],
            (times[i].as_micros() as f32 / overall.as_micros() as f32) * 100f32
        )?;
    }
    writeln!(results, "\nOverall Time: {overall:?}")?;
    writeln!(
        results,
        "\nSummed Time: {:?}",
        times.iter().fold(Duration::new(0, 0), |sum, x| sum + *x)
    )?;

    println!("{results}");

    // update the Readme. First 6 lines are kept as is
    let mut readme = String::new();
    if let Ok(content) = std::fs::read_to_string("README.md") {
        let header = content.lines().take(6);
        for line in header {
            writeln!(readme, "{line}").expect("Unable to write to string");
        }
    }
    write!(readme, "\n{results}")?;
    std::fs::write("README.md", readme).expect("Unable to write README.md");
    Ok(())
}
