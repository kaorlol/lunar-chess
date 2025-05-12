use anyhow::{Context as _, Result};
use clap::Parser;
use rand::{Rng, seq::IndexedRandom as _};
use std::time::Instant;

const PIECES: [&str; 12] = ["r", "n", "b", "q", "k", "p", "R", "N", "B", "Q", "K", "P"];
const PIECE_THEME: [&str; 33] = [
	"neo",
	"game_room",
	"wood",
	"glass",
	"gothic",
	"classic",
	"metal",
	"bases",
	"neo_wood",
	"icy_sea",
	"club",
	"ocean",
	"newspaper",
	"space",
	"cases",
	"condal",
	"marble",
	"book",
	"alpha",
	"bubblegum",
	"dash",
	"graffiti",
	"light",
	"lolz",
	"luca",
	"maya",
	"modern",
	"nature",
	"neon",
	"sky",
	"tigers",
	"tournament",
	"vintage",
];

// TODO: Add opencv to sobel the images

fn random_fen_generation() -> String {
	let mut rng = rand::rng();
	let mut fen = String::new();

	for _ in 0..8 {
		let mut row = vec![None; 8];
		for i in row.iter_mut().take(8) {
			if rng.random_bool(0.5) {
				*i = PIECES.choose(&mut rng).copied();
			}
		}

		let mut empty_count = 0;
		for piece in row {
			match piece {
				Some(p) => {
					if empty_count > 0 {
						fen.push_str(&empty_count.to_string());
						empty_count = 0;
					}
					fen.push_str(p);
				}
				None => {
					empty_count += 1;
				}
			}
		}
		if empty_count > 0 {
			fen.push_str(&empty_count.to_string());
		}
		fen.push('/');
	}

	fen.pop();
	fen.push_str(" w KQkq - 0 1");
	fen
}

async fn generate_dynboard(save_to: String) -> Result<()> {
	let piece_theme = PIECE_THEME
		.choose(&mut rand::rng())
		.context("Failed to choose piece theme")?;

	let fen = random_fen_generation();
	let url =
		format!("https://www.chess.com/dynboard?fen={fen}&board=green&piece={piece_theme}&size=3");

	let response = reqwest::get(&url).await?;
	let status = response.status();
	if status.is_success() {
		let bytes = response.bytes().await?;
		let name = format!("{save_to}/{piece_theme}-{}.png", fen.replace("/", "_"));
		std::fs::write(name, &bytes)?;
	}

	Ok(())
}

#[derive(Parser)]
#[clap(name = "dynboard-gen", about = "Generate chess board images")]
struct Args {
	/// The directory to save the images to
	#[clap(short, long, default_value = "dynboards")]
	save_to: String,

	/// The number of images to generate
	#[clap(short, long, default_value = "10")]
	count: usize,
}

#[tokio::main]
async fn main() {
	let args = Args::parse();
	let save_to = args.save_to;
	let count = args.count;
	std::fs::create_dir_all(&save_to).unwrap_or_else(|_| {
		eprintln!("Failed to create directory: {save_to}");
		std::process::exit(1);
	});

	let start = Instant::now();
	let mut tasks = Vec::new();
	for _ in 0..count {
		let save_to = save_to.clone();
		tasks.push(tokio::spawn(async {
			if let Err(e) = generate_dynboard(save_to).await {
				eprintln!("Error generating dynboard: {e}");
			}
		}));
	}
	for task in tasks {
		if let Err(e) = task.await {
			eprintln!("Error in task: {e}");
		}
	}
	let duration = start.elapsed();
	eprintln!("Generated {count} dynboards in {}s", duration.as_secs_f32());
}
