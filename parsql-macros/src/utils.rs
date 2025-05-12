use regex::Regex;

/// Extracts field names from a WHERE clause.
/// 
/// # Arguments
/// * `input` - The WHERE clause string
/// 
/// # Returns
/// * `Vec<String>` - A vector of field names found in the WHERE clause
pub(crate) fn extract_fields_from_where_clause(input: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let re = Regex::new(r"\b(\w+)\s*=\s*\$").unwrap();
    for cap in re.captures_iter(input) {
        if let Some(field) = cap.get(1) {
            fields.push(field.as_str().to_string());
        }
    }
    fields
}

/// SQL sorgularındaki parametre sayılarını takip etmek için yardımcı veri yapısı.
/// Bu yapı sayesinde, generate edilen SQL ile SQL parametreleri her zaman senkronize olur.
pub(crate) struct SqlParamCounter {
    /// Şu anki parametre numarası (1'den başlar)
    current: usize,
}

impl SqlParamCounter {
    /// 1'den başlayan yeni bir sayaç oluşturur
    pub fn new() -> Self {
        Self { current: 1 }
    }
    
    /// Mevcut parametre numarasını döndürür ve sayacı bir artırır
    pub fn next(&mut self) -> usize {
        let current = self.current;
        self.current += 1;
        current
    }
    
    /// Mevcut parametre numarasını döndürür (artırmadan)
    pub fn current(&self) -> usize {
        self.current
    }
    
    /// Toplam parametre sayısını döndürür (current - 1)
    pub fn count(&self) -> usize {
        self.current - 1
    }
}

/// WHERE koşulundaki parametre numaralarını doğru şekilde atayan yardımcı fonksiyon.
/// Bu fonksiyon, bağımsız olarak kullanılabilir ve sayaç değerini dışarıdan alır.
pub(crate) fn number_where_clause_params(clause: &str, counter: &mut SqlParamCounter) -> String {
    clause.chars()
        .map(|c| {
            if c == '$' {
                // $ işaretinden sonra numara ekle
                let param_num = counter.next();
                format!("${}", param_num)
            } else {
                // Diğer karakterleri olduğu gibi bırak
                c.to_string()
            }
        })
        .collect::<String>()
}