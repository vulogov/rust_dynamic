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
* You can iterate over All dynamically-typed objects.
* You can create attributes attached to wrapped data.
* Basic math operation Add/Sub/Mul/Div supported for FLOAT and INTEGER types.
* String concatenation and multiplication supported.

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
| Value::from_dict() | Create dynamic object of type MAP and initialize it from HashMap<String, Value> |
| Value::dict() | Create dynamic empty object of type MAP  |
| Value::none() | Create dynamic object that wraps value of None  |
| Value::nodata() | Create dynamic object that contains no data |
| Value::now() | Return dynamic object of type TIME containing current number of nanosecods from UNIX_EPOCH |
| Value::exit() | Return dynamic object of type EXIT  |
| Value::conv() | Converting of the object of one type to another |

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
| Value::export_float() | Return Vec<f64> from Value object |


Example:

```rust
use rust_dynamic::value::Value;

// First we create a dynamically-typed value from a raw static value
let mut value = Value::from(42).unwrap();

// Then we can cast raw value back from the dynamic object
let raw_value = value.cast_integer().unwrap()
```

## How to serialize and deserialize dynamically-typed values

There are two serialization formats that rust_dynamic presently supports: JSON and Bincode.

| Function name | Description |
|---|---|
| Value::to_json() | Return JSON representation of dynamically-typed value |
| Value::to_binary() | Return Bincode representation of dynamically-typed value |
| Value::from_json() | Takes string containing JSON representation of the dynamically-typed object and return re-created Value object |
| Value::from_binary() | Takes Vec<u8> containing Bincode representation of the dynamically-typed object and return re-created Value object |
| Value.wrap() | Return a ENVELOPE object containing a Bincode representation of object |
| Value.unwrap() | If object is ENVELOPE unpack binary representation of enclosed object, recreate it and return |

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
| Value::map_value() | Execute function to each element of the LIST or to the value and return new Value |
| Value::map_float() | Execute function to each FLOAT element of the LIST or to the value and return new Value |
| Value::push() | Ether add a new value to the list, or return a new Value |

Example:

```rust
// First, we define a function which will cast value of f64 and apply f64.sin() operation
fn comp_sin(value: Value) -> Value {
    match value.data {
        Val::F64(f_val) => {
            return Value::from_float(f64::sin(f_val));
        }
        _ => return value,
    }
}
// Then we create a single value object (map could be epplied to ether list or single value object)
let mut v = Value::from(42.0).unwrap();
// Now v contains result of computation
v = v.map_value(comp_sin);
```

## How to use dynamically-typed objects as HashMap keys

```rust
use std::collections::HashMap;

// This call will create a key object. Key object can be of any supported type
let key = Value::from(42.0 as f64).unwrap();

// Then we are creating a HashMap
let mut h: HashMap<Value, String> = HashMap::new();

// and store a key->value association
h.insert(key, "value".to_string());
```

## How to iterate over dynamically-typed objects

```rust
let mut c = 0.0;
// Let's create a object of LIST type and push two elements into list
let v = Value::list()
        .push(Value::from(1.0 as f64).unwrap())
        .push(Value::from(41.0 as f64).unwrap());
// We can iterate over dynamically-typed object
for i in v {
    c += i.cast_float().unwrap();
}
```

## How to map over dynamically-typed objects

In this example we are applying f64::sin function to all iterable values of the dynamically-typed object

```rust
let mut v = Value::from(42.0).unwrap();
v = v.map_float(f64::sin);
```
## How to use a math operations with dynamically-typed objects

At this moment, only FLOAT and INTEGER objects supported math operations.

```rust
// Let's create x and y objects both of FLOAT type
let mut x = Value::from(1.0 as f64).unwrap();
let y = Value::from(41.0 as f64).unwrap();
// And perform math operation as usual
x = x + y;
```
## How to concatenate string objects

At this moment, only STRING object supports that operation.

```rust
// Let's create x and y objects both of STRING type
let mut x = Value::from("Hello").unwrap();
// Then perform operation as usual, x shall be a object of STRING type containing string "Hello world"
let y = Value::from(" world").unwrap();
x = x + y;
```

## How to convert from String to Float

```rust
// This call returns the object of type FLOAT
let val = Value::from("42").unwrap().conv(FLOAT).unwrap();
```
