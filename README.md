# What in the Value ?

Rust's core strength is its ability to provide memory safety and performance guarantees at compile-time, which is achieved through its ownership and borrowing system. But ability for the developer to handle a dynamic data types defined at run-time for the applications written in Rust is still important endeavor and it is improving versatility of the application while providing an ability to handle a wide range of use cases.

Various interpreted DSL languages created in Rust uses run-time dynamic data type system created for the specific language. Excellent crate Serde also implements trait Value that's allow to handle dynamic datatypes in run-time. But none of those solutions is not perfect for a Rust developers as they ether focus on specific DSL or handling JSON values.

rust_dynamic is a crate, created for Rust language, implementing primitives that will be helping to Rust developer with the specific issue of handling dynamic data types defined at run-time. Currently, rust_dynamic supports following data types:

* Integer, internally represented as i64
* Float, internally represented as f64
* Boolean
* String, internally represented as String
* Pair, as a pair of dynamic values
* List, as a list of dynamic values
* Binary, as a Vector of u8 values

Dynamic values are wrapped and stored inside a Value structure and could be cast-able back into original Rust value.

```rust
use rust_dynamic::value::Value;

let mut value = Value::from(42).unwrap();
println!("Type of the stored value is {}", &value.type_name());
println!("Dynamic value of the value is {:?}", &value.cast_integer());

```

## What are the properties of dynamically-typed objects

* Value objects can wrap value of any supported type and they are functionally pure. Once you assigned the value, you can not change it, with one exception. Any attempts of change will result in generation of a new Value object. But you can push a new value to the object of List type, without re-creating a new object.
* All dynamically-typed objects are hash-able, so you can use them as the keys in HasMap objects.
* All dynamically-typed objects holds full information about stored data type.
* All dynamically-typed objects stores a unique object identifier.
* All dynamically-typed objects serialize-able to both JSON and a Bincode.

## How to create a dynamically-typed values

rust_dynamic crate supports a number of function-primitives that will take a raw value and return a wrapped Dynamic object.

| Function name | Description |
|---|---|
| Value::from_float() | Create dynamic object from f64 float number |
| Value::from_float32() | Create dynamic object from f32 float number |
| Value::from_integer() | Create dynamic object from i64 float number |
| Value::from_integer32() | Create dynamic object from i32 float number |
| Value::from_bool() | Create dynamic object from boolean value |
| Value::from_string() | Create dynamic object from Rust String |
| Value::from_str() | Create dynamic object from Rust &str |
| Value::from_bin() | Create dynamic object of type BINARY from Vec<u8> |
| Value::pair() | Create dynamic object of type PAIR from the pair of values |
| Value::list() | Create empty dynamic object of type LIST |
| Value::from_list() | Create dynamic object of type LIST and initialize it from Vec<Value> |
| Value::none() | Create dynamic object that wraps value of None  |
| Value::nodata() | Create dynamic object that contains no data |

There are generic function Value::from() that will automatically cast proper data type and ether return object or error message.

## How to cast Rust value from dynamically-typed values

rust_dynamic supports a number of casting functions that will try to extract wrapped value from the object that holds dynamically-typed value.

| Function name | Description |
|---|---|
| Value::cast_float() | Return f64 number from FLOAT object |
| Value::cast_integer() | Return i64 number from INTEGER object |
| Value::cast_bool() | Return boolean from BOOL object |
| Value::cast_string() | Return String from STRING object |
| Value::cast_bin() | Return Vec<u8> from BINARY object |
| Value::cast_list() | Return Vec<Value> from LIST object |

## How to serialize and deserialize dynamically-typed values

There are two serialization formats that rust_dynamic presently supports: JSON and Bincode.

| Function name | Description |
|---|---|
| Value::to_json() | Return JSON representation of dynamically-typed value |
| Value::to_binary() | Return Bincode representation of dynamically-typed value |
| Value::from_json() | Takes string containing JSON representation of the dynamically-typed object and return re-created Value object |
| Value::from_binary() | Takes Vec<u8> containing Bincode representation of the dynamically-typed object and return re-created Value object |

Example:

```rust
// This call will create a new dynamic value
let mut data = Value::from(42 as i64).unwrap();
// This call will serialize object to Bincode format
let bin_out = data.to_binary().unwrap();
// This will re-create an object from it's Bincode representation
let mut data2 = Value::from_binary(bin_out).unwrap();
```

## Functional operations with dynamically-typed values

While rust_dynamic crate is not strive to provide a full-featured functional interface to the dynamic values, some functionality that specific to a Functional programming has been implemented.

| Function name | Description |
|---|---|
| Value::bind() | Takes a reference to a function that accepts Value as a parameter, that function is called with passed current object and new Value object returned |
| Value::map() | Execute function to each element of the LIST or to the value and return new Value |
| Value::push() | Ether add a new value to the list, or return a new Value |
