use std::collections::HashMap;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculate: T,
    value: HashMap<u32, u32>
}


impl <T> Cacher<T>
where
    T: Fn(u32) -> u32 {

        fn new(calculate: T) -> Self {
            Self {
                calculate,
                value: HashMap::new()
            }
        }

        fn value(&mut self, value: u32) -> u32 {
            match self.value.get(&value) {
                Some(val) => *val,
                None => {
                    let result = (self.calculate)(value);
                    self.value.insert(value, result);
                    result
                }
            }
        }

    }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn call_with_different_values() {

        let x = 10;

        let mut c = Cacher::new( move |a| a+x);
        
        let v1 = c.value(1);
        let v2 = c.value(3);

        assert_eq!(v1, 11);
        assert_eq!(v2, 13);
    }

    #[test]
    fn check_closure_ownership() {
        let x = vec![1, 2, 3];
        let equal_to_x =  |z| z==x;
        println!("x is {:?}", x);
        assert!(equal_to_x(vec![1, 2, 3]));
    }
}