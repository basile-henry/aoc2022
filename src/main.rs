use assert_no_alloc::*;
use std::fmt::Write;
use std::path::{Path, PathBuf};

#[cfg(feature = "trace")]
use tracing_chrome::ChromeLayerBuilder;
#[cfg(feature = "trace")]
use tracing_subscriber::prelude::*;

#[cfg(all(debug_assertions, not(feature = "trace")))] // required when disable_release is set (default)
#[global_allocator]
static A: AllocDisabler = AllocDisabler;

const ALLOCATOR_CAPACITY: usize = 12 * 1024;

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

    let mut contents: [&[u8]; aoc2022::NUM_DAYS] = Default::default();

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
                let path: PathBuf = format!("inputs/day{day:0>2}.txt").into();
                *content = get_content_for_day(&path);
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
    });

    let io_span = tracing::span!(tracing::Level::TRACE, "Report");
    let _enter = io_span.enter();

    print!("{report}");

    Ok(())
}

// This purposefully leaks resources
// We should be able to open all 25 days input files at the same time and let the OS clean up the
// resources when the process exits
fn get_content_for_day(path: impl AsRef<Path>) -> &'static [u8] {
    let input_file = std::fs::File::open(path).unwrap();
    let mmap = unsafe { memmap2::Mmap::map(&input_file).unwrap() };
    let contents = unsafe { core::slice::from_raw_parts(mmap.as_ptr(), mmap.len()) };

    std::mem::forget(mmap);
    std::mem::forget(input_file);

    contents
}
