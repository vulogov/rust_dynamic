# What's new in 0.8.0

* More test cases
* Value::from_timestamp() takes u128 representing nanoseconds from epoch, making object of type TIME
* Value.cast_timestamp() casting TIME object to u128
* Value.get_time_as_datetime() - return UTC DateTime object handeled by chrono
* ==, >, <, >=, <= compare for TIME objects
* Value.map renamed to Value.map_value
