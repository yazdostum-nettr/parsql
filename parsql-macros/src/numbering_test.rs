/// WHERE koşulu içindeki $ işaretlerini numaralandıran fonksiyon
/// 
/// # Parametreler
/// 
/// * `clause` - İşlenmemiş WHERE koşulu
/// * `start_count` - Başlangıç numaralandırma değeri
/// 
/// # Dönüş Değeri
/// 
/// * Numaralandırılmış WHERE koşulu
pub fn number_where_clause_params(clause: &str, start_count: usize) -> String {
    let mut count = start_count;
    
    clause.chars()
        .map(|c| {
            if c == '$' {
                // $ işaretinden sonra numara ekle
                let new_char = format!("${}", count);
                count += 1;
                new_char
            } else {
                // Diğer karakterleri olduğu gibi bırak
                c.to_string()
            }
        })
        .collect::<String>()
}

/// Sorunlu senaryo: state >= $ ifadesinin state >= $11 olarak dönüştürülmesi
pub fn process_where_clause(where_clause: Option<&str>, start_count: usize) -> String {
    let mut count = start_count;

    where_clause
        .map(|clause| {
            clause.chars()
                .enumerate()
                .map(|(_, c)| {
                    if c == '$' {
                        // $ işaretinden sonra numara ekle
                        let new_char = format!("${}", count);
                        count += 1;
                        new_char
                    } else {
                        // Diğer karakterleri olduğu gibi bırak
                        c.to_string()
                    }
                })
                .collect::<String>()
        })
        .unwrap_or_else(|| "".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_numbering() {
        let result = number_where_clause_params("id = $", 1);
        assert_eq!(result, "id = $1");
    }
    
    #[test]
    fn test_multiple_params() {
        let result = number_where_clause_params("id = $ AND name = $", 1);
        assert_eq!(result, "id = $1 AND name = $2");
    }
    
    #[test]
    fn test_custom_start_count() {
        let result = number_where_clause_params("id = $", 5);
        assert_eq!(result, "id = $5");
    }
    
    #[test]
    fn test_process_where_clause_none() {
        let result = process_where_clause(None, 1);
        assert_eq!(result, "");
    }
    
    #[test]
    fn test_process_where_clause_with_param() {
        let result = process_where_clause(Some("state >= $"), 1);
        assert_eq!(result, "state >= $1");
    }
    
    #[test]
    fn test_process_where_clause_with_custom_start() {
        let result = process_where_clause(Some("state >= $"), 11);
        assert_eq!(result, "state >= $11");
    }
    
    #[test]
    fn test_process_problem_scenario() {
        // Bu test, sorunun neden oluştuğunu gösteriyor
        let mut count = 11; // Başka bir yerden değiştirilmiş olabilir
        let where_clause = Some("state >= $");
        
        let result = where_clause
            .map(|clause| {
                clause.chars()
                    .enumerate()
                    .map(|(_, c)| {
                        if c == '$' {
                            let new_char = format!("${}", count);
                            count += 1;
                            new_char
                        } else {
                            c.to_string()
                        }
                    })
                    .collect::<String>()
            })
            .unwrap_or_else(|| "".to_string());
        
        assert_eq!(result, "state >= $11"); // Sorunlu durum
    }
} 