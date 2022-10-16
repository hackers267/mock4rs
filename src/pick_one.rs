use rand::Rng;

/// To pick a random element from a list.
/// 从一个列表中随机挑选出一个元素
///
/// # Arguments
///
/// - list: 实现了`Copy` trait 的列表
///
/// returns 列表中的一个元素
///
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
