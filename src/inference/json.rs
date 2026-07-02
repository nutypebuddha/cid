#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, JsonValue::Bool(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, JsonValue::Number(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, JsonValue::Str(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, JsonValue::Array(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, JsonValue::Object(_))
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            JsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonValue::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&Vec<(String, JsonValue)>> {
        match self {
            JsonValue::Object(o) => Some(o),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(pairs) => {
                pairs.iter().find(|(k, _)| k == key).map(|(_, v)| v)
            }
            _ => None,
        }
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key).and_then(|v| v.as_str())
    }

    pub fn get_number(&self, key: &str) -> Option<f64> {
        self.get(key).and_then(|v| v.as_number())
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(|v| v.as_bool())
    }

    pub fn get_array(&self, key: &str) -> Option<&Vec<JsonValue>> {
        self.get(key).and_then(|v| v.as_array())
    }
}

pub struct JsonParser {
    input: Vec<char>,
    pos: usize,
}

impl JsonParser {
    pub fn new(input: &str) -> Self {
        JsonParser {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn parse(input: &str) -> Result<JsonValue, String> {
        let mut parser = JsonParser::new(input);
        parser.parse_value()
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();
        
        if self.pos >= self.input.len() {
            return Err("Unexpected end of input".to_string());
        }

        match self.input[self.pos] {
            'n' => self.parse_null(),
            't' | 'f' => self.parse_bool(),
            '"' => self.parse_string(),
            '[' => self.parse_array(),
            '{' => self.parse_object(),
            '-' | '0'..='9' => self.parse_number(),
            c => Err(format!("Unexpected character: {}", c)),
        }
    }

    fn parse_null(&mut self) -> Result<JsonValue, String> {
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos].is_alphabetic() {
            self.pos += 1;
        }
        let word: String = self.input[start..self.pos].iter().collect();
        if word == "null" {
            Ok(JsonValue::Null)
        } else {
            Err(format!("Expected 'null', got '{}'", word))
        }
    }

    fn parse_bool(&mut self) -> Result<JsonValue, String> {
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos].is_alphabetic() {
            self.pos += 1;
        }
        let word: String = self.input[start..self.pos].iter().collect();
        match word.as_str() {
            "true" => Ok(JsonValue::Bool(true)),
            "false" => Ok(JsonValue::Bool(false)),
            _ => Err(format!("Expected 'true' or 'false', got '{}'", word)),
        }
    }

    fn parse_string(&mut self) -> Result<JsonValue, String> {
        if self.input[self.pos] != '"' {
            return Err("Expected '\"'".to_string());
        }
        self.pos += 1;
        let mut result = String::new();

        while self.pos < self.input.len() {
            match self.input[self.pos] {
                '"' => {
                    self.pos += 1;
                    return Ok(JsonValue::Str(result));
                }
                '\\' => {
                    self.pos += 1;
                    if self.pos >= self.input.len() {
                        return Err("Unexpected end of string escape".to_string());
                    }
                    match self.input[self.pos] {
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        '/' => result.push('/'),
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        'b' => result.push('\u{0008}'),
                        'f' => result.push('\u{000C}'),
                        'u' => {
                            self.pos += 1;
                            if self.pos + 4 > self.input.len() {
                                return Err("Invalid unicode escape".to_string());
                            }
                            let hex: String = self.input[self.pos..self.pos + 4].iter().collect();
                            let code = u32::from_str_radix(&hex, 16)
                                .map_err(|e| format!("Invalid unicode hex: {}", e))?;
                            if let Some(c) = char::from_u32(code) {
                                result.push(c);
                            }
                            self.pos += 3;
                        }
                        c => return Err(format!("Invalid escape character: {}", c)),
                    }
                }
                c => result.push(c),
            }
            self.pos += 1;
        }

        Err("Unterminated string".to_string())
    }

    fn parse_number(&mut self) -> Result<JsonValue, String> {
        let start = self.pos;
        
        if self.input[self.pos] == '-' {
            self.pos += 1;
        }

        if self.pos >= self.input.len() {
            return Err("Unexpected end of number".to_string());
        }

        if self.input[self.pos] == '0' {
            self.pos += 1;
        } else if self.input[self.pos].is_ascii_digit() {
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        } else {
            return Err("Invalid number".to_string());
        }

        if self.pos < self.input.len() && self.input[self.pos] == '.' {
            self.pos += 1;
            if self.pos >= self.input.len() || !self.input[self.pos].is_ascii_digit() {
                return Err("Invalid number after decimal".to_string());
            }
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }

        if self.pos < self.input.len() && (self.input[self.pos] == 'e' || self.input[self.pos] == 'E') {
            self.pos += 1;
            if self.pos < self.input.len() && (self.input[self.pos] == '+' || self.input[self.pos] == '-') {
                self.pos += 1;
            }
            if self.pos >= self.input.len() || !self.input[self.pos].is_ascii_digit() {
                return Err("Invalid number in exponent".to_string());
            }
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }

        let num_str: String = self.input[start..self.pos].iter().collect();
        let num = num_str.parse::<f64>().map_err(|e| format!("Invalid number: {}", e))?;
        Ok(JsonValue::Number(num))
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        if self.input[self.pos] != '[' {
            return Err("Expected '['".to_string());
        }
        self.pos += 1;
        let mut array = Vec::new();

        self.skip_whitespace();
        if self.pos < self.input.len() && self.input[self.pos] == ']' {
            self.pos += 1;
            return Ok(JsonValue::Array(array));
        }

        loop {
            let value = self.parse_value()?;
            array.push(value);
            self.skip_whitespace();

            if self.pos >= self.input.len() {
                return Err("Unterminated array".to_string());
            }

            match self.input[self.pos] {
                ',' => self.pos += 1,
                ']' => {
                    self.pos += 1;
                    return Ok(JsonValue::Array(array));
                }
                c => return Err(format!("Expected ',' or ']', got '{}'", c)),
            }
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        if self.input[self.pos] != '{' {
            return Err("Expected '{{'".to_string());
        }
        self.pos += 1;
        let mut pairs = Vec::new();

        self.skip_whitespace();
        if self.pos < self.input.len() && self.input[self.pos] == '}' {
            self.pos += 1;
            return Ok(JsonValue::Object(pairs));
        }

        loop {
            self.skip_whitespace();
            let key = self.parse_string()?.as_str().unwrap_or("").to_string();
            
            self.skip_whitespace();
            if self.pos >= self.input.len() || self.input[self.pos] != ':' {
                return Err("Expected ':'".to_string());
            }
            self.pos += 1;

            let value = self.parse_value()?;
            pairs.push((key, value));

            self.skip_whitespace();
            if self.pos >= self.input.len() {
                return Err("Unterminated object".to_string());
            }

            match self.input[self.pos] {
                ',' => self.pos += 1,
                '}' => {
                    self.pos += 1;
                    return Ok(JsonValue::Object(pairs));
                }
                c => return Err(format!("Expected ',' or '}}', got '{}'", c)),
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }
}

pub fn parse_json(input: &str) -> Result<JsonValue, String> {
    JsonParser::parse(input)
}

pub fn stringify(value: &JsonValue) -> String {
    match value {
        JsonValue::Null => "null".to_string(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Number(n) => {
            if *n == (*n as i64) as f64 && n.abs() < 1e15 {
                format!("{}", *n as i64)
            } else {
                format!("{}", n)
            }
        }
        JsonValue::Str(s) => {
            let mut result = String::from('"');
            for c in s.chars() {
                match c {
                    '"' => result.push_str("\\\""),
                    '\\' => result.push_str("\\\\"),
                    '\n' => result.push_str("\\n"),
                    '\r' => result.push_str("\\r"),
                    '\t' => result.push_str("\\t"),
                    '\u{0008}' => result.push_str("\\b"),
                    '\u{000C}' => result.push_str("\\f"),
                    c if c.is_control() => {
                        result.push_str(&format!("\\u{:04x}", c as u32));
                    }
                    c => result.push(c),
                }
            }
            result.push('"');
            result
        }
        JsonValue::Array(arr) => {
            let mut result = String::from('[');
            for (i, item) in arr.iter().enumerate() {
                if i > 0 {
                    result.push(',');
                }
                result.push_str(&stringify(item));
            }
            result.push(']');
            result
        }
        JsonValue::Object(pairs) => {
            let mut result = String::from('{');
            for (i, (key, value)) in pairs.iter().enumerate() {
                if i > 0 {
                    result.push(',');
                }
                result.push_str(&stringify(&JsonValue::Str(key.clone())));
                result.push(':');
                result.push_str(&stringify(value));
            }
            result.push('}');
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_null() {
        assert_eq!(parse_json("null").unwrap(), JsonValue::Null);
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse_json("true").unwrap(), JsonValue::Bool(true));
        assert_eq!(parse_json("false").unwrap(), JsonValue::Bool(false));
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_json("42").unwrap(), JsonValue::Number(42.0));
        assert_eq!(parse_json("-3.14").unwrap(), JsonValue::Number(-3.14));
        assert_eq!(parse_json("1e10").unwrap(), JsonValue::Number(1e10));
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(parse_json("\"hello\"").unwrap(), JsonValue::Str("hello".to_string()));
        assert_eq!(parse_json("\"line\\none\"").unwrap(), JsonValue::Str("line\none".to_string()));
    }

    #[test]
    fn test_parse_array() {
        assert_eq!(parse_json("[1,2,3]").unwrap(), JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
        ]));
    }

    #[test]
    fn test_parse_object() {
        let json = "{\"name\":\"test\",\"value\":42}";
        let result = parse_json(json).unwrap();
        assert_eq!(result.get_str("name"), Some("test"));
        assert_eq!(result.get_number("value"), Some(42.0));
    }

    #[test]
    fn test_stringify() {
        let value = JsonValue::Object(vec![
            ("name".to_string(), JsonValue::Str("test".to_string())),
            ("value".to_string(), JsonValue::Number(42.0)),
        ]);
        let json = stringify(&value);
        assert!(json.contains("\"name\":\"test\""));
        assert!(json.contains("\"value\":42"));
    }
}
