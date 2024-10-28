# What's new in 0.32.0

* Support for JSON object
* Wrapping value of JSON during bincode/envelope
* Support ```push``` for JSON arrays
* Support ```len``` for JSON
* Support ```+``` for JSON acting as JSON merge
* New method: ```Value::json()``` created JSON object
* New method: ```Value::cast_json()``` Casting serde_json::Value out of Value::JSON object
* New method: ```Value::cast_value_to_json()``` converting Value object to wrapped Value::JSON
* New method: ```Value::cast_json_to_value()``` Convert wrapped Vaue::JSON object to appropriate Value
