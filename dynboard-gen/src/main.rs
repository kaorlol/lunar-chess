use anyhow::{Context as _, Result};
use clap::Parser;
use opencv::{
	core::{self, AlgorithmHint::ALGO_HINT_DEFAULT, CV_64F, Size},
	imgcodecs, imgproc,
	prelude::*,
};
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

		let image = imgcodecs::imdecode(&bytes.as_ref(), imgcodecs::IMREAD_UNCHANGED)?;
		let sobel_image = sobel(&image)?;
		imgcodecs::imwrite(&name, &sobel_image, &core::Vector::<i32>::new())?;
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
async fn main() -> Result<()> {
	let args = Args::parse();
	let save_to = args.save_to;
	let count = args.count;
	std::fs::create_dir_all(&save_to)
		.with_context(|| format!("Failed to create directory: {save_to}"))?;

	let start = Instant::now();
	let mut tasks = Vec::new();
	for _ in 0..count {
		let save_to = save_to.clone();
		tasks.push(tokio::spawn(async {
			generate_dynboard(save_to)
				.await
				.with_context(|| "Failed to generate dynboard")
		}));
	}
	for task in tasks {
		task.await??;
	}
	let duration = start.elapsed();
	println!("Generated {count} dynboards in {}s", duration.as_secs_f32());
	Ok(())
}

fn sobel(input_image_mat: &Mat) -> Result<Mat, opencv::Error> {
	let mut gray_image = Mat::default();
	if input_image_mat.channels() == 3 {
		imgproc::cvt_color(
			input_image_mat,
			&mut gray_image,
			imgproc::COLOR_BGR2GRAY,
			0,
			ALGO_HINT_DEFAULT,
		)?;
	} else if input_image_mat.channels() == 4 {
		imgproc::cvt_color(
			input_image_mat,
			&mut gray_image,
			imgproc::COLOR_BGRA2GRAY,
			0,
			ALGO_HINT_DEFAULT,
		)?;
	} else {
		gray_image = input_image_mat.clone();
	}

	let mut blurred_image = Mat::default();
	let blur_kernel_size = Size::new(3, 3);
	imgproc::gaussian_blur(
		&gray_image,
		&mut blurred_image,
		blur_kernel_size,
		0.0,
		0.0,
		core::BORDER_DEFAULT,
		ALGO_HINT_DEFAULT,
	)?;

	let mut grad_x = Mat::default();
	let mut grad_y = Mat::default();

	imgproc::sobel(
		&blurred_image,
		&mut grad_x,
		CV_64F,
		1,
		0,
		3,
		1.0,
		0.0,
		core::BORDER_DEFAULT,
	)?;

	imgproc::sobel(
		&blurred_image,
		&mut grad_y,
		CV_64F,
		0,
		1,
		3,
		1.0,
		0.0,
		core::BORDER_DEFAULT,
	)?;

	let mut abs_grad_x = Mat::default();
	let mut abs_grad_y = Mat::default();
	core::convert_scale_abs(&grad_x, &mut abs_grad_x, 1.0, 0.0)?;
	core::convert_scale_abs(&grad_y, &mut abs_grad_y, 1.0, 0.0)?;

	let mut grad_magnitude = Mat::default();
	core::add_weighted(
		&abs_grad_x,
		0.5,
		&abs_grad_y,
		0.5,
		0.0,
		&mut grad_magnitude,
		-1,
	)?;

	let mut thresholded_edges = Mat::default();
	imgproc::threshold(
		&grad_magnitude,
		&mut thresholded_edges,
		0.0,
		255.0,
		imgproc::THRESH_BINARY + imgproc::THRESH_OTSU,
	)?;

	let mut final_image = Mat::default();
	core::bitwise_not(&thresholded_edges, &mut final_image, &core::no_array())?;

	Ok(final_image)
}
