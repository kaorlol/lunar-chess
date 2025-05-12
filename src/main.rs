use opencv::{
	core::{self, BORDER_DEFAULT, CV_8U, CV_64F, Size},
	imgproc,
	prelude::*,
};
use xcap::{Monitor, XCapError, XCapResult};

mod draw_arrow;

#[tokio::main]
async fn main() {

	// let monitor = monitor().unwrap();
	// let image = monitor.capture_image().unwrap();
	// image.save("screenshot.png").unwrap();

	// let files = std::fs::read_dir("dynboards").unwrap();
	// for file in files {
	// 	let file = file.unwrap();
	// 	if file.path().extension().unwrap() == "png" {
	// 		let image = opencv::imgcodecs::imread(
	// 			file.path().to_str().unwrap(),
	// 			opencv::imgcodecs::IMREAD_COLOR,
	// 		)
	// 		.unwrap();
	// 		let sobel = sobel(&image).unwrap();
	// 		opencv::imgcodecs::imwrite(
	// 			file.path().to_str().unwrap(),
	// 			&sobel,
	// 			&opencv::core::Vector::new(),
	// 		)
	// 		.unwrap();
	// 	}
	// }

	// draw_arrow::draw((1085, 1173), (1082, 994)).unwrap();
}

fn monitor() -> XCapResult<Monitor> {
	let monitors = Monitor::all()?;
	monitors
		.into_iter()
		.find(|monitor| monitor.is_primary().unwrap_or(false))
		.ok_or_else(|| XCapError::Error("Could not find primary monitor".to_string()))
}

fn sobel(input_image_mat: &Mat) -> Result<Mat, opencv::Error> {
	let mut gray_image = Mat::default();
	if input_image_mat.channels() == 3 {
		imgproc::cvt_color(input_image_mat, &mut gray_image, imgproc::COLOR_BGR2GRAY, 0)?;
	} else if input_image_mat.channels() == 4 {
		imgproc::cvt_color(
			input_image_mat,
			&mut gray_image,
			imgproc::COLOR_BGRA2GRAY,
			0,
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
