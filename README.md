# What in the Value ?

Rust's primary strength lies in its ability to ensure memory safety and performance guarantees at compile time, which is accomplished through its ownership and borrowing system. However, the capability for developers to manage dynamically defined data types at runtime in Rust applications is not just a pursuit, but a key to enhancing the application's adaptability and versatility, enabling it to address a diverse array of use cases.

Specific interpreted DSL languages devised in Rust utilize a runtime dynamic data type system customized for the specific language. In addition, the reputable crate Serde plays a significant role by incorporating the trait Value, thereby providing a strong sense of security and confidence to Rust developers in the management of dynamic data types at runtime. However, these solutions have limitations, as they cater to specific DSLs or the handling of JSON values.

rust_dynamic, a crate developed for the Rust language, encompasses primitives designed to assist Rust developers in managing dynamically defined data types at runtime. Presently, rust_dynamic supports the following data types:

* Integer, internally represented as i64
* Float, internally represented as f64
* Boolean
* String, internally represented as String
* Lambda functions/Curry functions and pointed on functions
* Pair, as a pair of dynamic values
* List, as a list of dynamic values
* Matrix, as a 2D collection of of vynamic values allocated in a logical grid.
* Binary, as a Vector of u8 values
* Nanosecond grade timestamp
* Any dynamically-typed object wrapped in envelope
* Complex numbers for i64 and f64
* Maps
* Associations
* None
* NODATA
* Error
* Metrics as a Vector of TIMESTAMP->F64 samples
* JSON - dynamic Value object containing JSON

Dynamic values are wrapped and stored inside a Value structure and could be castable back into the original Rust value.

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
* LIST-type objects supports car/cdr/at/head and tail functions returning the values or the splices of the list

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
| Value::operator() | Create OPERATOR with OPCODE and OPVALUE |
| Value::context() | Create CONTEXT with random ID |
| Value::named_context() | Create named CONTEXT  |
| Value::list() | Create empty dynamic object of type LIST |
| Value::matrix() | Create empty dynamic object of type MATRIX |
| Value::from_list() | Create dynamic object of type LIST and initialize it from Vec<Value> |
| Value::from_dict() | Create dynamic object of type MAP and initialize it from HashMap<String, Value> |
| Value::dict() | Create dynamic empty object of type MAP  |
| Value::json() | Create dynamic empty object for storing JSON values  |
| Value::none() | Create dynamic object that wraps value of None  |
| Value::nodata() | Create dynamic object that contains no data |
| Value::now() | Return dynamic object of type TIME containing current number of nanosecods from UNIX_EPOCH |
| Value::exit() | Return dynamic object of type EXIT  |
| Value::metrics() | Return dynamic object of type METRICS for 128 samples  |
| Value::metrics_n(n) | Return dynamic object of type METRICS for n samples  |
| Value::lambda() | Return dynamic object of type LAMBDA   |
| Value::to_lambda() | Return dynamic object of type LAMBDA created from Vector  |
| Value::curry() | Return dynamic object of type CURRY. Basic and empty  |
| Value::conditional() | Return dynamic object of type CONDITIONAL. Basic and empty  |
| Value::conditional_from_dict() | Return dynamic object of type CURRY. Initialized from the HashTable  |
| Value::ptr_curry() | Return dynamic object of type CURRY. Containing pointed to the function  |
| Value::lambda_curry() | Return dynamic object of type CURRY. Containing lambda function  |
| Value::result() | Return dynamic object of type RESULT   |
| Value::to_result() | Return dynamic object of type RESULT created from Vector  |
| Value::from_metrics() | Return dynamic object created from Vec of Metrics  |
| Value::from_complex_int() | Return dynamic object created from Complex<i64>  |
| Value::from_complex_float() | Return dynamic object created from Complex<f64>  |
| Value::conv() | Converting of the object of one type to another |
| Value::from_matrix() | Return dynamic object of MATRIX type created from Vec<Vec<Value>>  |

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
| Value::cast_list() | Return Vec<Value> from LIST/PAIR object |
| Value::cast_result() | Return Vec<Value> from RESULT object |
| Value::cast_lambda() | Return Vec<Value> from LAMBDA object |
| Value::cast_pair() | Return Vec<Value> from PAIR object |
| Value::cast_pair_x() | Return Value from PAIR X object |
| Value::cast_pair_y() | Return Value from PAIR Y object |
| Value::cast_metrics() | Return Vec<Metric> from METRICS object |
| Value::cast_fifo() | Return next Value from FIFO object |
| Value::cast_queue() | Return next Value from QUEUE object |
| Value::cast_dict() | Return HashMap<String,Value> from MAP,INFO,CONFIG,ASSOCIATION objects |
| Value::cast_complex_int() | Return Complex<i64> from CINTEGER object |
| Value::cast_complex_float() | Return Complex<f64> from CFLOAT object |
| Value::export_float() | Return Vec<f64> from Value object |
| Value::cast_json() | Cast serde_json::Value from JSON object |
| Value::cast_json_to_value() | Cast Dynamic value from JSON object |
| Value::cast_matrix() | Return Vec<Vec<Value>> from MATRIX |


Example:

```rust
use rust_dynamic::value::Value;

// First we create a dynamically-typed value from a raw static value
let mut value = Value::from(42).unwrap();

// Then we can cast raw value back from the dynamic object
let raw_value = value.cast_integer().unwrap()
```

And here is an example of creating and casting JSON data

```rust
//
// First, we will create a Value of JSON type
//
let data = Value::json(serde_json::json!(42));
//
// Now, we are converting JSON values to Dynamic types
//
let value = data.cast_json_to_value()
```


## How to set attributes to the Value

Value attributes is a Vector of the values that stored in the Value object. You can assign any number of the Value objects stored as attributes of the Value object. Attributes are serailize-able and wrap-able.

| Function name | Description |
|---|---|
| Value.attr(Vec<Value>) | Set vector of values as an attributes of the Value object |
| Value.attr_add(Value) | Add a Value to the end of attributes list of the Value object. Returns duplicate of Value object |
| Value.attr_merge(Vec<Value>) | Merge Vector of Values and current attributes of the Value object. Returns duplicate of Value object |
| Value.attr_len() | Return number of attributes of the Value object |

Example:

```rust
// Create object
let v = Value::from(42 as i64).unwrap()
                // Set the attributes of the object
                .attr(vec![Value::from(41.0 as f64).unwrap()])
                // And merge some extra attributes. The first attribute now have a value of 42.0
                .attr_merge(vec![Value::from(42.0 as f64).unwrap()]);
```

## How to serialize and deserialize dynamically-typed values

There are two serialization formats that rust_dynamic presently supports: JSON and Bincode. If you are serialize to Biuncode, specail provisioning (wrapping) for objects holding JSON's will be done.

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

## Integration of dynamically-typed values with serde_json crate.

Crate rust_dynamic is capable to export Value object to  serde_json::value::Value

| Function name | Description |
|---|---|
| Value.as_json_value() | Function converts Value object to serde_json::value::Value |

Example:

```rust
// This call will create a Value object of type PAIR and on-the-fly exports it
// to the serde_json::value::Value object that can be processed as any other Value object created by serde_json crate.
let v = Value::pair(Value::from_int(1), Value::from_int(2)).as_json_value();
```


## Functional operations with dynamically-typed values

While rust_dynamic crate is not strive to provide a full-featured functional interface to the dynamic values, some functionality that specific to a Functional programming has been implemented.

| Function name | Description |
|---|---|
| Value.pop() | Returns last pushed value to the (LIST|RESULT|FIFO) or None |
| Value.bind() | Takes a reference to a function that accepts Value as a parameter, that function is called with passed current object and new Value object returned |
| Value::bind_values() | Takes a reference to a function that accepts two Values as a parameter, and two values. New Value object returned that is result of programmatic binding of the values |
| Value.fmap() | Execute function to each element of the LIST or to the value and return new Value |
| Value.map_float() | Execute function to each FLOAT element of the LIST or to the value and return new Value |
| Value.push() | Ether add a new value to the list, or return a new Value |
| Value.push_inplace() | Push new Value to supported Value (LIST, QUEUE, FIFO, RESULT, METRIC) or return Error |
| Value.maybe() | Takes a function which is if returns true, Value.maybe() returns value, if false Value::none() |
| Value::left_right() | Takes a function which is if returns true, and a references on two Values. Value::left_right() returns clone of first value, if function return true, second othewise |
| Value::freduce() | Takes two parameters - function which takes two values and returning a single value and initial value, reducing dynamic value to a single value by applying function to all elements |


Example of mapping:

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

Example of binding

```rust
// First, let's create a "binding function" which will takes two Value objects and return a new Value
// In our example, simple sum() will be performed
fn sum_of_values(v1: Value, v2: Value) -> Value {
    v1 + v2
}
// Then let's create two values
let v1 = Value::from(41.0 as f64).unwrap();
let v2 = Value::from(1.0 as f64).unwrap();
// Value referred as _s_ now contains value of 42.0
let s = Value::bind_values(sum_of_values, v1, v2);
```

Example of maybe()

```rust
// First, we create a function that will check if v is float and is 42
fn if_value_is_42(v: &Value) -> bool {
    if v.cast_float().unwrap() == 42.0 {
        return true;
    }
    false
}
// And because it is, v is object of Value(42.0)
// Otherwise it would be Value::none()
let v = Value::from(42.0 as f64).unwrap()
        .maybe(if_value_is_42);
```

Example of reducing

```rust
// First, we shall create a list of the values
let mut v = Value::from_list(
    vec![Value::from_float(41.0 as f64),
    Value::from_float(1.0 as f64)]);
// Then reduce this list applying function that sums "accumulator" and current value
v = v.freduce(
    |x: Value,y: Value| -> Value { x+y },
    Value::from_float(0.0 as f64));
// This function shall return a single FLOAT value wrapping number 42.0
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

# Functional features of rust_dynamic

## Create and use of Applicative

You can create an Applicative that is wrapping a function and apply a Value object with wrapped value to Applicative. For example:

```rust
// First, as usual, we are defining function, that will be wrapped in Applicative
fn comp_sin(value: Value) -> Value {
    match value.data {
        Val::F64(f_val) => {
            return Value::from_float(f64::sin(f_val));
        }
        _ => return value,
    }
}

// Create applicative and wrap a function
let sin = Applicative::new(comp_sin);
// Then apply a Value to a wrapped function
let res = sin.apply(Value::from(42.0).unwrap());
```

# Neural Network features of rust_dynamic

## Conversion Values to Tensors

You can easily convert dynamic value into a Tensor. At this moment, we support this feature for INTEGER, FLOAT and STRING Values

```rust
let value = Value::from(42.0);
match value {
  Ok(val) => {
    match val.tensor() {
      Ok(tensor) => {
        // Here will be a tensor
      }
      Err(_) => todo!();
    }
  }
  Err(_) => todo!();
}
```
