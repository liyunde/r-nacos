pub struct StringUtils;

impl StringUtils {
    pub fn is_empty(s: &str) -> bool {
        s.is_empty()
    }

    pub fn eq(a: &str, b: &str) -> bool {
        a == b
    }

    pub fn like(a: &str, b: &str) -> Option<usize> {
        a.rfind(b)
    }

    pub fn is_none_or_empty(v: &Option<String>) -> bool {
        if let Some(v) = &v {
            if v.is_empty() {
                true
            } else {
                false
            }
        } else {
            true
        }
    }

    ///
    /// 空字符串转为None
    pub fn map_not_empty(v: Option<String>) -> Option<String> {
        if let Some(v) = &v {
            if v.is_empty() {
                return None;
            }
        }
        v
    }
}
