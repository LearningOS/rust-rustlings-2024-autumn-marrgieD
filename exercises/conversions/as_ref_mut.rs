// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn byte_counter<T>(arg: T) -> usize 
where
    T: AsRef<str>,
{
    arg.as_ref().as_bytes().len() //T: AsRef<str>: 这一部分表示泛型参数 T 必须实现 AsRef<str> trait。AsRef<str> 是一个标准库中的 trait，表示可以将类型 T 引用为 &str，即 T 可以被转换为字符串引用。AsRef 提供了一个方法 as_ref()，用于将类型 T 转换为 &str。

    arg.as_ref(): 这个调用会将 T 转换为 &str 类型，因为 T 实现了 AsRef<str>。
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn char_counter<T>(arg: T) -> usize 
where
    T: AsRef<str>,
{
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.
fn num_sq<T>(arg: &mut T) 
where
    T: AsRef<str>,
{
    // TODO: Implement the function body.
    *arg.as_mut() *= *arg.as_mut(); //左侧: *arg.as_mut()，表示获取 arg 所引用的值（通过可变引用）。


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
