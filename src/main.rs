#[allow(unused_imports)]
// use mouse_rs::move_absolute_recursive{types::keys::Keys, Mouse}; 
use std::process;
use std::{alloc::System, process::exit, thread::sleep, time::Duration};
// use hookmap::device::{self, mouse};
use inputbot::{KeySequence,MouseCursor, KeybdKey::*, MouseButton::*};

fn main() {
	// loop{
		QKey.bind(||{std::process::exit(0)});
		LControlKey.bind(||{
		let mouse_position=inputbot::MouseCursor::pos();
		MouseCursor::move_abs(mouse_position.0+384, mouse_position.1);
		sleep(Duration::from_millis(1));
		MouseCursor::move_abs(mouse_position.0-384, mouse_position.1);
		sleep(Duration::from_millis(1));
		MouseCursor::move_abs(mouse_position.0, mouse_position.1);
		// println!("x :{:?} y : {:?}", mouse_position.0,mouse_position.0+384);
	});
		Numrow1Key.bind(|| {
			for x in 0..=600 {
				MouseCursor::move_abs(x, 300);
				sleep(Duration::from_millis(1));
			}
		});
		Numrow2Key.bind(|| {
			MouseCursor::move_rel(100, 100);
			sleep(Duration::from_millis(1));
		});
inputbot::handle_input_events();}	
	// 	let  pos:(i32,i32) = mouse::get_position();
	// 	if pos.0<384{
	// 	let  left_mouse_x:i32=0 ;
	// 	let  right_mouse_x:i32=pos.0+384;
	// 		mouse::move_absolute_recursive(right_mouse_x,pos.1);
	// 		mouse::move_absolute_recursive(left_mouse_x,pos.1);
	// 		mouse::move_absolute_recursive(pos.0,pos.1);
			
	// 		continue;
	// 	}
		
	// 	if pos.0>1151{
	// 		let  left_mouse_x:i32=pos.0-384 ;
	// 		let  right_mouse_x:i32=1535;
	// 		mouse::move_absolute_recursive(right_mouse_x,pos.1);
	// 		mouse::move_absolute_recursive(left_mouse_x,pos.1);
	// 		mouse::move_absolute_recursive(pos.0,pos.1);
	// 		continue;
	// 	}
	// 	else {
	// 		let left_mouse_x:i32=pos.0-384;
	// 		let right_mouse_x:i32=pos.0+384;
	// 		mouse::move_absolute_recursive(right_mouse_x,pos.1);
	// 		mouse::move_absolute_recursive(left_mouse_x,pos.1);
	// 		mouse::move_absolute_recursive(pos.0,pos.1);
	// 		continue;
			

	// }
// }
// }


