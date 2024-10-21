pub fn plus(a: u32, b: u32) -> u64 {
    (a + b).into()
}

#[cfg(test)]
mod test {
    #[test]
    fn a() {
        assert_eq!(6, crate::plus(2, 4));
    }
}
