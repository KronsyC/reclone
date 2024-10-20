
#[macro_export]
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


#[cfg(test)]
mod test{
    use crate::cloned;
    #[test]
    fn clone_arc(){
        use std::sync::Arc;

        let foo = Arc::new(10u32);
        let bar = Arc::new(20u32);

        cloned!(foo, bar, {
            std::mem::drop(foo);
            std::mem::drop(bar);
        });

        println!("Variables: {foo:?} {bar:?}");
    }
}
