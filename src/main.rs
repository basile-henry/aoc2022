use assert_no_alloc::*;
use std::fmt::Write;
use std::path::Path;

#[cfg(feature = "trace")]
use tracing_chrome::ChromeLayerBuilder;
#[cfg(feature = "trace")]
use tracing_subscriber::prelude::*;

// #[cfg(all(debug_assertions, not(feature = "trace")))] // required when disable_release is set (default)
// #[global_allocator]
// static A: AllocDisabler = AllocDisabler;

const ALLOCATOR_CAPACITY: usize = 70 * 1024;

fn main() -> std::io::Result<()> {
    #[cfg(feature = "trace")]
    let (chrome_layer, _guard) = ChromeLayerBuilder::new().build();
    #[cfg(feature = "trace")]
    tracing_subscriber::registry().with(chrome_layer).init();

    let main_span = tracing::span!(tracing::Level::TRACE, "main");
    let _enter = main_span.enter();

    let mut args = std::env::args();

    let program_name = args.next().unwrap();
    let cli_day = if let Some(day) = args.next() {
        match day.parse::<u8>() {
            Ok(day) => Some(day),
            Err(_) => {
                print!(
                    r#"
Usage:
    {program_name} [DAY] [INPUT_PATH]

Defaults to all the days when none specified
"#
                );
                std::process::exit(1);
            }
        }
    } else {
        None
    };
    let cli_input_path = args.next();

    let io_span = tracing::span!(tracing::Level::TRACE, "Allocator / IO");
    let io_span = io_span.enter();

    #[allow(unused)]
    let mut bump = bumpalo::Bump::with_capacity(ALLOCATOR_CAPACITY);
    bump.set_allocation_limit(Some(0));

    let mut contents: [&str; 25] = Default::default();

    if let Some(day) = cli_day {
        contents[day as usize - 1] = get_content_for_day(
            cli_input_path
                .as_deref()
                .unwrap_or(format!("inputs/day{day:0>2}.txt").as_str()),
        );
    } else {
        for (day, content) in contents.iter_mut().enumerate() {
            let day = day + 1;
            tracing::span!(tracing::Level::TRACE, "day").in_scope(|| {
                *content = get_content_for_day(format!("inputs/day{day:0>2}.txt").as_str());
            });
        }
    }

    let mut report = String::with_capacity(1024);

    drop(io_span);

    macro_rules! day {
        ($mod:ident, $day:expr) => {
            if cli_day.unwrap_or($day) == $day {
                let day = $day;
                let (part1, part2) = aoc2022::$mod::$mod(contents[day - 1]);
                write!(report, "{day}: {part1} {part2}\n").unwrap();
            }
        };
        ($mod:ident, $day:expr, $bump:expr) => {
            if cli_day.unwrap_or($day) == $day {
                $bump.reset();
                let day = $day;
                let (part1, part2) = aoc2022::$mod::$mod(&$bump, contents[day - 1]);
                write!(report, "{day}: {part1} {part2}\n").unwrap();
            }
        };
    }

    assert_no_alloc(|| {
        day!(day01, 1);
        day!(day02, 2);
        day!(day03, 3);
        day!(day04, 4);
        day!(day05, 5, bump);
        day!(day06, 6);
        day!(day07, 7, bump);
        day!(day08, 8, bump);
        day!(day09, 9, bump);
        day!(day10, 10, bump);
        day!(day11, 11, bump);
        day!(day12, 12, bump);
        day!(day18, 18, bump);
        day!(day24, 24, bump);
        day!(day25, 25, bump);
    });

    let io_span = tracing::span!(tracing::Level::TRACE, "Report");
    let _enter = io_span.enter();

    print!("{report}");

    Ok(())
}

// This purposefully leaks strings
// We should be able to hold all 25 days in memory quite easily
fn get_content_for_day(path: impl AsRef<Path>) -> &'static str {
    match std::fs::read_to_string(&path) {
        Ok(content) => Box::leak(content.into_boxed_str()),
        Err(err) => {
            eprintln!("Warn: {err} on path {}", path.as_ref().display());
            ""
        }
    }
}
