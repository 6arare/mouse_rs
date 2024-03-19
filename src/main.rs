#[allow(unused_imports)]
#[allow(d_codsolmove_absolute_recursivee)]
// use mouse_rs::move_absolute_recursive{types::keys::Keys, Mouse}; 
use std::process;
// use clearscreen;
use std::{thread,time};
use hookmap::device::{self, mouse};

// struct Position{
// 	x:i32,
// 	lmove_absolute_recursive_recursive:i32,
// }
// mouse.move_absolute_recursive_recursive(100,100).expect("Unable to move Mouse");
 // mouse.move_absolute_recursive_recursive(pos1.x,pos1.y).expect("Unable to move Mouse");
 // mouse.press(&Keys::LEFT).expect("Unable to press button");
 // mouse.release(&Keys::LEFT).expect("Unable to release button");

 // mouse.move_absolute_recursive_recursive(pos2.x,pos2.y).expect("Unable to move Mouse");
 // mouse.press(&Keys::LEFT).expect("Unable to press button");
 // mouse.release(&Keys::LEFT).expect("Unable to release button");

 // mouse.move_absolute_recursive_recursive(pos3.x,pos3.y).expect("Unable to move Mouse");
 // mouse.press(&Keys::LEFT).expect("Unable to press button");
 // mouse.release(&Keys::LEFT).expect("Unable to release button");
// let mouse = Mouse::new();
// let position=mouse.get_position().unwrap();
// let pos2 = Position{x:805 ,y:524};
// let pos3 = Position{x:1335 ,y:526};
fn main() {
// thread::spawn(||{
	loop{
		let  pos:(i32,i32) = mouse::get_position();
		if pos.0<384{
		let  left_mouse_x:i32=0 ;
		let  right_mouse_x:i32=pos.0+384;
			// println!("{:?}   {:?}   {:?}",left_mouse_x,pos.0,right_mouse_x);
			mouse::move_absolute_recursive(right_mouse_x,pos.1);
			mouse::move_absolute_recursive(left_mouse_x,pos.1);
			mouse::move_absolute_recursive(pos.0,pos.1);
			
			continue;
		}
		
		if pos.0>1151{
			let  left_mouse_x:i32=pos.0-384 ;
			let  right_mouse_x:i32=1535;
			// println!("{:?}   {:?}   {:?}",left_mouse_x,pos.0,right_mouse_x);
			mouse::move_absolute_recursive(right_mouse_x,pos.1);
			mouse::move_absolute_recursive(left_mouse_x,pos.1);
			mouse::move_absolute_recursive(pos.0,pos.1);
			continue;
		}
		else {
			let left_mouse_x:i32=pos.0-384;
			let right_mouse_x:i32=pos.0+384;
			// println!("{:?}   {:?}   {:?}",left_mouse_x,pos.0,right_mouse_x);
			mouse::move_absolute_recursive(right_mouse_x,pos.1);
			mouse::move_absolute_recursive(left_mouse_x,pos.1);
			mouse::move_absolute_recursive(pos.0,pos.1);
			continue;
			

	}}}
// thread::sleep(time::Duration::from_millis(1));
	
// });
// loop{
// }
// clearscreen::clear().expect("failed to clear the screen");
// mouse::move_absolute_recursive_recursive(pos.0,pos.1);

// mouse.move_absolute_recursive(position.x, position.y);
// mouse.move_absolute_recursive(left_mouse_x,position.y);
//mouse.move_absolute_recursive(right_mouse_x,position.y);
// mouse.press(&Keys::LEFT).expect("Unable to press button");
// mouse.release(&Keys::LEFT).expect("Unable to release button");



