#[derive(Debug)]
struct MyStr {
    data: String
}

impl MyStr {
    fn new(data: &str) -> Self {
        Self {
            data: String::from(data)
        }
    }

    fn iter(&self) -> MyStrIter {
        MyStrIter { slice: &self.data[..] }
    }
}

struct MyStrIter<'a> {
    slice: &'a str
}

impl<'a> Iterator for MyStrIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {

        if !self.slice.is_empty() {

            let (data, rem ) = self.slice.split_at(1);
            self.slice = rem;
            Some(data)
        } else {
            return None;
        }
    }
}

impl<'a> IntoIterator for &'a MyStr {
    type Item = &'a str;
    type IntoIter = MyStrIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let str1 = MyStr::new("hello");

        let mut striter = str1.iter();
        assert_eq!(Some("h"), striter.next());
        assert_eq!(Some("e"), striter.next());
        assert_eq!(Some("l"), striter.next());
        assert_eq!(Some("l"), striter.next());
        assert_eq!(Some("o"), striter.next());
        assert_eq!(None, striter.next());
    }

    #[test]
    fn for_loop() {

        let data = "hello";
        let str1 = MyStr::new(data);

        for (idx, char) in str1.iter().enumerate() {
            assert_eq!(char, &data[idx..idx+1]);
        }
    }

    #[test]
    fn for_loop_into() {

        let data = "hello";
        let str1 = MyStr::new(data);

        let mut count = 0;
        for char in str1.into_iter() {
            assert_eq!(char, &data[count..count+1]);
            count += 1;
        }
        assert_eq!("hello", str1.data);
    }
}
