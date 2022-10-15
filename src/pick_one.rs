use rand::Rng;

pub fn pick_one<T>(list: &[T]) -> T
where
    T: Copy,
{
    let range = 0..list.len();
    list[rand::thread_rng().gen_range(range)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick_one() {
        let list = vec![1, 2, 3, 4, 5, 6, 7];
        let result = pick_one(&list);
        assert!(list.contains(&result));
    }
}
