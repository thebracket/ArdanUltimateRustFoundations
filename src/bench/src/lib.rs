#![feature(test)]
extern crate test;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[bench]
    fn bench_add(b: &mut Bencher) {
        b.iter(|| add(2, 4));
    }
}
