use std::thread;
use std::time::{Duration, Instant};

use console::{Emoji, style};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use rand::Rng;
use rand::prelude::IndexedRandom;

static PACKAGES: &[&str] = &[
    "fs-events",
    "my-awesome-module",
    "emoji-speaker",
    "wrap-ansi",
    "stream-browserify",
    "acorn-dynamic-import",
];

static COMMANDS: &[&str] = &[
    "cmake .",
    "make",
    "make clean",
    "gcc foo.c -o foo",
    "gcc bar.c -o bar",
    "./helper.sh rebuild-cache",
    "make all-clean",
    "make test",
];

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

pub fn main() {
    let mut rng = rand::rng();
    let started = Instant::now();
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    let deps = 1232;
    let pb = ProgressBar::new(deps);
    // for _ in 0..deps {
    //     thread::sleep(Duration::from_micros(1));
    //     pb.inc(1);
    //  }
    pb.finish_and_clear();
    let шкала = MultiProgress::new();
    let handles: Vec<_> = (0..4u32)
        .map(|i| {
            let счётчик = rng.random_range(30..80);
            let pb = шкала.add(ProgressBar::new(счётчик));
            pb.set_style(spinner_style.clone());
            pb.set_prefix(format!("[{}/?]", i + 1));
            thread::spawn(move || {
                let mut rng = rand::rng();
                //let pkg = PACKAGES.choose(&mut rng).unwrap();
                for _ in 0..счётчик {
                    // let cmd = COMMANDS.choose(&mut rng).unwrap();
                    thread::sleep(Duration::from_micros(rng.random_range(1..10)));
                    //pb.set_message(format!("{pkg}: {cmd}"));
                    pb.inc(1);
                }
                pb.finish_with_message("ожидание...");
            })
        })
        .collect();
    for h in handles {
        let _ = h.join();
    }
    шкала.clear().unwrap();

    println!("{} Завершено {}", SPARKLE, HumanDuration(started.elapsed()));
}
