fn main() {
    // You can optionally experiment here.
    let mut x: [i32; 4] = [1, 2, 4, 5];
    x[0] = 10;
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x; // explain this line
        y.push(42);
        let z = &mut x; // and this one
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
