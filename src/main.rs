use assert_no_alloc::*;

#[cfg(debug_assertions)] // required when disable_release is set (default)
#[global_allocator]
static A: AllocDisabler = AllocDisabler;

const ALLOCATOR_CAPACITY: usize = 20 * 1024;

fn main() -> std::io::Result<()> {
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

    let mut bump = bumpalo::Bump::with_capacity(ALLOCATOR_CAPACITY);
    bump.set_allocation_limit(Some(0));

    macro_rules! day {
        ($mod:ident, $day:expr, $path:expr) => {
            if cli_day.unwrap_or($day) == $day {
                let input_file = std::fs::File::open(
                    cli_input_path.as_ref().map(|s| s.as_str()).unwrap_or($path),
                )?;
                let contents = unsafe { memmap2::Mmap::map(&input_file)? };
                bump.reset();
                let (part1, part2) = assert_no_alloc(|| aoc2016::$mod::solve(&bump, &contents[..]));
                println!("{}: {part1:?} {part2:?}", $day);
            }
        };
    }

    day!(day01, 1, "inputs/day01.txt");

    Ok(())
}
