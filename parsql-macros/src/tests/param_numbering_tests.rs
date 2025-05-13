#[cfg(test)]
mod param_numbering_tests {
    use crate::{number_where_clause_params, SqlParamCounter};

    /// Test basic parameter numbering with new counter
    #[test]
    fn test_basic_numbering() {
        let mut counter = SqlParamCounter::new();
        let result = number_where_clause_params("id = $", &mut counter);
        assert_eq!(result, "id = $1");
        assert_eq!(counter.current(), 2); // current should be 2 after using 1
    }

    /// Test multiple parameters
    #[test]
    fn test_multiple_params() {
        let mut counter = SqlParamCounter::new();
        let result = number_where_clause_params("id = $ AND name = $", &mut counter);
        assert_eq!(result, "id = $1 AND name = $2");
        assert_eq!(counter.current(), 3); // current should be 3 after using 1 and 2
    }

    /// Test sequential numbering for WHERE and HAVING
    #[test]
    fn test_sequential_numbering() {
        let mut counter = SqlParamCounter::new();
        
        // İlk WHERE cümlesi
        let where_result = number_where_clause_params("id = $", &mut counter);
        assert_eq!(where_result, "id = $1");
        assert_eq!(counter.current(), 2); // current now should be 2
        
        // İkinci HAVING cümlesi, otomatik olarak 2'den devam etmeli
        let having_result = number_where_clause_params("status = $", &mut counter);
        assert_eq!(having_result, "status = $2");
        assert_eq!(counter.current(), 3);
    }

    /// Test correct numbering with multiple clauses
    #[test]
    fn test_complex_query_numbering() {
        let mut counter = SqlParamCounter::new();
        
        // WHERE cümlesi
        let where_result = number_where_clause_params("state >= $ AND created_at > $", &mut counter);
        assert_eq!(where_result, "state >= $1 AND created_at > $2");
        assert_eq!(counter.current(), 3);
        
        // HAVING cümlesi
        let having_result = number_where_clause_params("count(*) > $", &mut counter);
        assert_eq!(having_result, "count(*) > $3");
        assert_eq!(counter.current(), 4);
        
        // SQL sorgusu şöyle olmalı:
        // "... WHERE state >= $1 AND created_at > $2 ... HAVING count(*) > $3 ..."
    }
} 