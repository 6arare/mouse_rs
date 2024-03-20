#[allow(unused_imports)]
#[allow(unused_variablesd_codsolmove_absolute_recursivee)]
// use mouse_rs::move_absolute_recursive{types::keys::Keys, Mouse}; 
use std::process;
use std::{thread,time};
use hookmap::device::{self, mouse};

fn main() {
	loop{
		let  pos:(i32,i32) = mouse::get_position();
		if pos.0<384{
		let  left_mouse_x:i32=0 ;
		let  right_mouse_x:i32=pos.0+384;
			mouse::move_absolute_recursive(right_mouse_x,pos.1);
			mouse::move_absolute_recursive(left_mouse_x,pos.1);
			mouse::move_absolute_recursive(pos.0,pos.1);
			
			continue;
		}
		
		if pos.0>1151{
			let  left_mouse_x:i32=pos.0-384 ;
			let  right_mouse_x:i32=1535;
			mouse::move_absolute_recursive(right_mouse_x,pos.1);
			mouse::move_absolute_recursive(left_mouse_x,pos.1);
			mouse::move_absolute_recursive(pos.0,pos.1);
			continue;
		}
		else {
			let left_mouse_x:i32=pos.0-384;
			let right_mouse_x:i32=pos.0+384;
			mouse::move_absolute_recursive(right_mouse_x,pos.1);
			mouse::move_absolute_recursive(left_mouse_x,pos.1);
			mouse::move_absolute_recursive(pos.0,pos.1);
			continue;
			

	}}}


