# What's new in 0.9.0

* More test cases
* Value.fmap coulf be applied to MAP/CONFIG/INFO/ASSOCIATION datatypes
* Applicative::new() - allows to define applicative functor that will take a Value in Applicative.apply() and apply wrapped function to a wrapped object
* New trait Context for running functions over Values within specific context. You have to implement your own context. NullContext is provided as an example and for the testing. All functions called within context must be of type CtxAppFn 
