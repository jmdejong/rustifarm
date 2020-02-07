

use std::any::Any;


#[macro_export]
macro_rules! assemblage {
	($name:ident { $($arg:ident : $argt:ident ),* } ; $( $comp:expr ),* ) => {
		#[derive(Debug, Clone, Default)]
		pub struct $name {$(
			pub $arg : $argt
		)* }
		impl Assemblage for $name {
			fn build<'a>(&self, mut builder: specs::EntityBuilder<'a>) -> specs::EntityBuilder<'a>{
				$(
					let $arg = &self.$arg;
				)*
				$(
					builder = specs::Builder::with(builder, $comp);
				)*
				builder
			}
			
			#[allow(unused_variables, unused_mut)]
			fn init_from_json(&mut self, mut args: Vec<serde_json::Value>, kwargs: std::collections::HashMap<String, serde_json::Value>) {
				$(
					if args.len() > 0 {
						let val = args.remove(0);
						if let Some(actual_val) = super::unpack_json!($argt, val) {
							self.$arg = actual_val;
						}
					}
				)*
				$(
					if let Some(val) = kwargs.get(stringify!($arg)) {
						if let Some(actual_val) = super::unpack_json!($argt, val) {
							self.$arg = actual_val;
						}
					}
				)*
			}
		}
	}
}

#[macro_export]
macro_rules! unpack_json {
	(String, $val: ident) => {
		if let Some(txt) = $val.as_str(){
			Some(txt.to_string())
		} else {
			None
		}
	}
}


pub trait Assemblage: Send + Sync + Any {
	fn build<'a>(&self, builder: specs::EntityBuilder<'a>) -> specs::EntityBuilder<'a>;
	fn init_from_json(&mut self, args: Vec<serde_json::Value>, kwargs: std::collections::HashMap<String, serde_json::Value>);
}


