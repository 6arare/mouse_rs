//TODO: create a gui 
#[allow(unused_imports)]
// use mouse_rs::move_relative{types::keys::Keys, Mouse}; 
use std::{process::exit, thread::sleep, time::Duration};
use hookmap::device::{self, mouse};
// use inputbot::{MouseCursor, KeybdKey::*};

fn main() {
	loop{
		// QKey.bind(||{std::process::exit(0)});
		// LControlKey.bind(||{
		// // let mouse_position=inputbot::MouseCursor::pos();
		// MouseCursor::move_rel(384, 0);
		// sleep(Duration::from_millis(10));
		// MouseCursor::move_rel(-768, 0);
		// sleep(Duration::from_millis(10));
		// MouseCursor::move_rel(384, 0);	
		// sleep(Duration::from_millis(10));
		// // MouseCursor::move_rel(mouse_position.0, mouse_position.1);
		// sleep(Duration::from_millis(10));
		// println!("x :{:?} y : {:?}", mouse_position.0,mouse_position.0+384);
	// });
		
// inputbot::handle_input_events();}	
		// let  pos:(i32,i32) = mouse::get_position();
	// 	if pos.0<384{
	// 	let  left_mouse_x:i32=0 ;
	// 	let  right_mouse_x:i32=pos.0+384;
			mouse::move_relative(384,0);
				sleep(Duration::from_millis(10));
			mouse::move_relative(-768,0);
				sleep(Duration::from_millis(10));	
			mouse::move_relative(384,0);
				sleep(Duration::from_millis(10));	
	// 		mouse::move_relative(pos.0,pos.1);
			
	// 		continue;
		}
		
	// 	if pos.0>1151{
	// 		let  left_mouse_x:i32=pos.0-384 ;
	// 		let  right_mouse_x:i32=1535;
	// 		mouse::move_relative(right_mouse_x,pos.1);
	// 		mouse::move_relative(left_mouse_x,pos.1);
	// 		mouse::move_relative(pos.0,pos.1);
	// 		continue;
	// 	}
	// 	else {
	// 		let left_mouse_x:i32=pos.0-384;
	// 		let right_mouse_x:i32=pos.0+384;
	// 		mouse::move_relative(right_mouse_x,pos.1);
	// 		mouse::move_relative(left_mouse_x,pos.1);
	// 		mouse::move_relative(pos.0,pos.1);
	// 		continue;
			

	}



	
