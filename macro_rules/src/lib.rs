
// This macro is created based on a crust of rust epsiode by jonhoo. 
#[macro_export]
macro_rules! avec {
    ($($x:expr),*) => {
        {
            const LENGTH: usize = count!($($x),*);
            #[allow(unused_mut)]
            let mut temp_vec = Vec::with_capacity(LENGTH);
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
    ($($x:expr,)*) => {{
        avec! [$($x),*]
    }};
    
}

#[macro_export]
macro_rules! count {
    (SUBST $x: expr) => {
        ()
    };
    ($($x: expr),*) => {
        <[()]>::len(&[$(count! (SUBST $x)),*])
    };
}


#[cfg(test)]
mod tests {
    #[test]
    fn creates_empty_vec() {
        let v: Vec<i32> = avec![];
        assert!(v.is_empty());
    }

    #[test]
    fn creates_vec_wit_two_items() {
        let v: Vec<i32> = avec![1, 2];
        assert!(v.len() == 2);
        assert!(v[0] == 1);
        assert!(v[1] == 2);

    }
    
    #[test]
    fn creates_vec_and_accepts_trailing_comma() {
        let v = avec![1, 2,];
        assert!(v.len() == 2);
        assert!(v[0] == 1);
        assert!(v[1] == 2);
    }
}
