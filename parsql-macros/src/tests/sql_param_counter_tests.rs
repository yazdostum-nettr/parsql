#[cfg(test)]
mod sql_param_counter_tests {
    use crate::crud_impl::SqlParamCounter;

    #[test]
    fn test_counter_starts_at_one() {
        let counter = SqlParamCounter::new();
        assert_eq!(counter.current(), 1);
    }

    #[test]
    fn test_counter_increment() {
        let mut counter = SqlParamCounter::new();
        assert_eq!(counter.next(), 1);
        assert_eq!(counter.current(), 2);
        assert_eq!(counter.next(), 2);
        assert_eq!(counter.current(), 3);
    }

    #[test]
    fn test_counter_count() {
        let mut counter = SqlParamCounter::new();
        assert_eq!(counter.count(), 0); // Henüz hiç sayı kullanılmadı
        
        counter.next(); // 1
        assert_eq!(counter.count(), 1); // Bir sayı kullanıldı
        
        counter.next(); // 2
        counter.next(); // 3
        assert_eq!(counter.count(), 3); // Üç sayı kullanıldı
    }
} 