use assert_no_alloc::*;

#[cfg(feature = "trace")]
use tracing_chrome::ChromeLayerBuilder;
#[cfg(feature = "trace")]
use tracing_subscriber::prelude::*;

#[cfg(all(debug_assertions, not(feature = "trace")))] // required when disable_release is set (default)
#[global_allocator]
static A: AllocDisabler = AllocDisabler;

const ALLOCATOR_CAPACITY: usize = 40 * 1024;

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

    let mut bump = tracing::info_span!("Setup allocator")
        .in_scope(|| bumpalo::Bump::with_capacity(ALLOCATOR_CAPACITY));
    bump.set_allocation_limit(Some(0));

    macro_rules! day {
        ($mod:ident, $day:expr, $path:expr) => {
            if cli_day.unwrap_or($day) == $day {
                let io_span = tracing::span!(tracing::Level::TRACE, "IO");
                let _enter = io_span.enter();

                let input_file = std::fs::File::open(
                    cli_input_path.as_ref().map(|s| s.as_str()).unwrap_or($path),
                )?;
                let contents = unsafe { memmap2::Mmap::map(&input_file)? };
                bump.reset();
                let (part1, part2) = assert_no_alloc(|| aoc2022::$mod::$mod(&bump, &contents[..]));
                println!("{}: {part1:?} {part2:?}", $day);
            }
        };
    }

    day!(day01, 1, "inputs/day01.txt");
    day!(day02, 2, "inputs/day02.txt");
    day!(day03, 3, "inputs/day03.txt");

    Ok(())
}
