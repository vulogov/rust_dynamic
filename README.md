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

let mut value = Value::from(42);
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
