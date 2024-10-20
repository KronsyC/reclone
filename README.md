# Reclone


Minimal macro library to help with cloning objects into move closures


I've found that when building applications, you commonly have a set of Arcs representing   
state of some sort, and need to move it into a move closure 

This is annoying and leads to code like this:


```rust

let foo = ...;
let bar = ...;
let baz = ...;

{
    let foo = foo.clone();
    let bar = bar.clone();
    let baz = baz.clone();

    tokio::spawn(async move{
        // Use the variables
    });
}

```

Now with reclone, this becomes, in my opinion, a little neater and less repetitive


```rust

use reclone::cloned;

let foo = ...;
let bar = ...;
let baz = ...;


cloned!(foo, bar, baz, {
    tokio::spawn(async move{
        // Use the variables
    });
});


```


If you can't justify bringing this in as a dependency, you're welcome to directly inline   
the code in your codebase 


```rust
macro_rules! cloned {
    ($($var : ident ,)* $scope : block) => {
        {
            $(
                let $var = $var.clone();
            )*
            $scope
        }
    }
}
```


