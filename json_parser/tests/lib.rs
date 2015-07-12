extern crate json_parser;

#[test]
fn simple_json() {
	let test_object = json_parser::json::JsonObject::new("{\"key0\":\"value0\",\"key1\":\"value1\"}");
	
	assert_eq!("value0", test_object.get_root().get_child("key0").get_string());
}


#[test]
fn complex_json() {
	let test_object = json_parser::json::JsonObject::new("{\"complex_key\":{\"key0\":\"value0\",\"key1\":\"value1\"}");
	
	assert_eq!("value0", test_object.get_root().get_child("complex_key").get_child("key0").get_string());
}