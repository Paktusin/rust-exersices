fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x:Vec<i32> = Vec::new();
        let y: &mut Vec<i32> = &mut x;
        let z: &mut Vec<i32> = y;
        z.push(42);
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
