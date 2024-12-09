pub struct List<T> {
    root: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        struct Iter<'a, T> {
            node: Option<&'a Node<T>>,
        }

        impl<'a, T> std::iter::Iterator for Iter<'a, T> {
            type Item = &'a T;

            fn next(&mut self) -> Option<Self::Item> {
                let Some(node) = self.node else {
                    return None;
                };
                self.node = node.next.as_ref().map(|node| &**node);
                Some(&node.value)
            }
        }

        Iter {
            node: self.root.as_ref().map(|node| &**node),
        }
    }
}

#[macro_export]
macro_rules! list {
    ($($e:expr),*) => ({
        let mut values = vec![$($e),*];
        let mut root = None;
        while let Some(value) = values.pop() {
            let node = Node {
                value,
                next: root,
            };
            root = Some(Box::new(node));
        }
        List { root }
    })
}

impl<T> std::fmt::Debug for List<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        let mut values = self.iter();
        if let Some(v) = values.next() {
            write!(f, "{v:?}")?;
        }
        for v in values {
            write!(f, ", {v:?}")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_list() {
    let data = list![1, 2, 3, 3, 4, 5, 6];
    println!("{data:?}");
}
