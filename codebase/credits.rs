extern crate sdl_rust;

// libs needed
use std::time::Duration;
use std::thread;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use sdl_rust::SDLCore;
use sdl_rust::Demo;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use sdl2::render::Texture;

// set window vars 
const TITLE: &str = "Farnan's Farmers";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

pub struct SDL01 {
	core: SDLCore,
}



impl Demo for SDL01 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL01{ core })
	}


	fn run(&mut self) -> Result<(), String> {
		self.core.wincan.set_blend_mode(BlendMode::Blend);
		let texture_creator = self.core.wincan.texture_creator();
		let r = Rect::new((0) as i32, (0) as i32, CAM_W, CAM_H);
		self.core.wincan.set_draw_color(Color::RGBA(255, 255, 255, 255));
		self.core.wincan.clear();

		// paths for group images
		let img1 = "images/nightmare_fuel.png";
		let img2 = "images/small_bg.png";
		let img3 = "images/tuxdoge.png";
		let img4 = "images/nightmare_fuel.png";
		let img5 = "images/hello_world_win.png";
		let img6 = "images/small_bg.png";
		let img7 = "images/tuxdoge.png";
		let mut images = [img1, img2, img3, img4, img5, img6, img7];

		// itterate through images and display fade 
		for img in 0..images.len(){
			fade(self, texture_creator.load_texture(images[img])?, r);
		}
		thread::sleep(Duration::from_millis(300));

		Ok(())
	}

}


// method to fade in and out 
fn fade(window: &mut SDL01, ms: Texture, r:  Rect) -> Result<(), String> {
	// fade in 
	let mut i = 0;
	while i < 254 {
		window.core.wincan.clear();
		window.core.wincan.copy(&ms, None, None)?;
		window.core.wincan.set_draw_color(Color::RGBA(255, 255, 255, 255-i));
		window.core.wincan.fill_rect(r)?;
		window.core.wincan.present();
		thread::sleep(Duration::from_millis(5));
		i = i + 1;
	}

	// fade out 
	i = 0;
	while i < 254 {
		window.core.wincan.clear();
		window.core.wincan.copy(&ms, None, None)?;
		window.core.wincan.set_draw_color(Color::RGBA(255, 255, 255, i));
		window.core.wincan.fill_rect(r)?;
		window.core.wincan.present();
		thread::sleep(Duration::from_millis(5));
		i = i + 1;
	}
	Ok(())
}

fn main() {
	sdl_rust::runner(TITLE, SDL01::init);
}
