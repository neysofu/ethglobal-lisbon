use fvm_sdk;

pub fn add(left: usize, right: usize) -> usize {
    fvm_sdk::initialize();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
