use rdma_sys::*;

#[cfg(test)]
mod tests {
    #[test]
    fn rdma_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}