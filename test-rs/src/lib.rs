use egnitely_client::Context;
use std::error::Error;
use serde_json::{json, Value};

pub async fn handler(_ctx: Context, input:Value) -> Result<Value, Box<dyn Error>> {

	println!("Input Data: {:?}", input);
	//TODO: Implement your function

	Ok(json!({
		"message": "function executed successfully"
	}))
}
