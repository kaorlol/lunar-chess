// use opencv::{
// 	core::{self, AlgorithmHint::ALGO_HINT_DEFAULT, BORDER_DEFAULT, CV_8U, CV_64F, Size},
// 	imgproc,
// 	prelude::*,
// };
use pixels::{Error, Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::macos::WindowAttributesExtMacOS as _;
use winit::window::{Window, WindowId};
use xcap::{Monitor, XCapError, XCapResult};

mod draw_arrow;

#[derive(Default)]
struct App {
	window: Option<Window>,
	pixel_buffer: Vec<u32>,
}

impl ApplicationHandler for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		self.window = Some(
			event_loop
				.create_window(
					Window::default_attributes()
						.with_transparent(true)
						.with_titlebar_transparent(true),
				)
				.unwrap(),
		);
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
		match event {
			WindowEvent::CloseRequested => {
				println!("The close button was pressed; stopping");
				event_loop.exit();
			}
			WindowEvent::RedrawRequested => {
				self.pixel_buffer.fill(0xFF000000);

				draw_arrow(&mut self.pixel_buffer, 800, 600);

				self.window.as_ref().unwrap().request_redraw();
			}
			_ => (),
		}
	}
}

fn draw_arrow(buffer: &mut Vec<u32>, width: usize, height: usize) {
	// Example: draw a simple arrow using pixel manipulation
	let arrow_color = 0xFFFFFFFF; // white

	// Draw the shaft
	for x in 100..150 {
		for y in 300..305 {
			buffer[x + y * width] = arrow_color;
		}
	}

	// Draw the arrowhead
	for x in 150..180 {
		for y in (285 + (x - 150))..(320 - (x - 150)) {
			buffer[x + y * width] = arrow_color;
		}
	}
}

// TODO: https://github.com/pykeio/ort/blob/main/examples/yolov8/yolov8.rs

fn main() {
	let event_loop = EventLoop::new().unwrap();

	// ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
	// dispatched any events. This is ideal for games and similar applications.
	event_loop.set_control_flow(ControlFlow::Poll);

	// ControlFlow::Wait pauses the event loop if no events are available to process.
	// This is ideal for non-game applications that only update in response to user
	// input, and uses significantly less power/CPU time than ControlFlow::Poll.
	event_loop.set_control_flow(ControlFlow::Wait);

	let mut pixel_buffer: Vec<u32> = vec![0; 800 * 600];

	let mut app = App::default();
	event_loop.run_app(&mut app).unwrap();

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

// fn sobel(input_image_mat: &Mat) -> Result<Mat, opencv::Error> {
// 	let mut gray_image = Mat::default();
// 	if input_image_mat.channels() == 3 {
// 		imgproc::cvt_color(
// 			input_image_mat,
// 			&mut gray_image,
// 			imgproc::COLOR_BGR2GRAY,
// 			0,
// 			ALGO_HINT_DEFAULT,
// 		)?;
// 	} else if input_image_mat.channels() == 4 {
// 		imgproc::cvt_color(
// 			input_image_mat,
// 			&mut gray_image,
// 			imgproc::COLOR_BGRA2GRAY,
// 			0,
// 			ALGO_HINT_DEFAULT,
// 		)?;
// 	} else {
// 		gray_image = input_image_mat.clone();
// 	}

// 	let mut blurred_image = Mat::default();
// 	let blur_kernel_size = Size::new(3, 3);
// 	imgproc::gaussian_blur(
// 		&gray_image,
// 		&mut blurred_image,
// 		blur_kernel_size,
// 		0.0,
// 		0.0,
// 		core::BORDER_DEFAULT,
// 		ALGO_HINT_DEFAULT,
// 	)?;

// 	let mut grad_x = Mat::default();
// 	let mut grad_y = Mat::default();

// 	imgproc::sobel(
// 		&blurred_image,
// 		&mut grad_x,
// 		CV_64F,
// 		1,
// 		0,
// 		3,
// 		1.0,
// 		0.0,
// 		core::BORDER_DEFAULT,
// 	)?;

// 	imgproc::sobel(
// 		&blurred_image,
// 		&mut grad_y,
// 		CV_64F,
// 		0,
// 		1,
// 		3,
// 		1.0,
// 		0.0,
// 		core::BORDER_DEFAULT,
// 	)?;

// 	let mut abs_grad_x = Mat::default();
// 	let mut abs_grad_y = Mat::default();
// 	core::convert_scale_abs(&grad_x, &mut abs_grad_x, 1.0, 0.0)?;
// 	core::convert_scale_abs(&grad_y, &mut abs_grad_y, 1.0, 0.0)?;

// 	let mut grad_magnitude = Mat::default();
// 	core::add_weighted(
// 		&abs_grad_x,
// 		0.5,
// 		&abs_grad_y,
// 		0.5,
// 		0.0,
// 		&mut grad_magnitude,
// 		-1,
// 	)?;

// 	let mut thresholded_edges = Mat::default();
// 	imgproc::threshold(
// 		&grad_magnitude,
// 		&mut thresholded_edges,
// 		0.0,
// 		255.0,
// 		imgproc::THRESH_BINARY + imgproc::THRESH_OTSU,
// 	)?;

// 	let mut final_image = Mat::default();
// 	core::bitwise_not(&thresholded_edges, &mut final_image, &core::no_array())?;

// 	Ok(final_image)
// }
