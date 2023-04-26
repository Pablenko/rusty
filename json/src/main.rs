use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

macro_rules! json {
    (null) => {
        Json::Null
    };
    ([ $( $element:tt ),* ]) => {
        Json::Array(vec![ $( json!($element) ),*])
    };
    ({ $( $key:tt : $value:tt ),* }) => {
        Json::Object(Box::new(vec![
            $( ($key.to_string(), json!($value)) ),*
        ].into_iter().collect()))
    };
    ($other:tt) => {
        Json::from($other)
    };
}

impl From<bool> for Json {
    fn from(value: bool) -> Json {
        Json::Boolean(value)
    }
}

impl From<String> for Json {
    fn from(value: String) -> Json {
        Json::String(value)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(value: &'a str) -> Json {
        Json::String(value.to_string())
    }
}

macro_rules! impl_from_number_for_json  {
    ( $ ( $t:ident )* ) => {
        $(
            impl From<$t> for Json {
                fn from(value: $t) -> Json {
                    Json::Number(value as f64)
                }
            }
        )*
    };
}

impl_from_number_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64);

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Json;

    #[test]
    fn test_json_null() {
        let json = json!(null);
        assert_eq!(json, Json::Null);
    }

    #[test]
    fn test_json_array() {
        let expected = Json::Object(Box::new(
            vec![(
                "array".to_string(),
                Json::Array(vec![
                    Json::Number(1.0),
                    Json::Number(2.0),
                    Json::Number(3.0),
                ]),
            )]
            .into_iter()
            .collect(),
        ));
        let json = json!({
            "array": [1, 2, 3]
        });
        assert_eq!(json, expected);
    }

    #[test]
    fn test_json_basic_types() {
        let expected = Json::Object(Box::new(
            vec![
                ("number".to_string(), Json::Number(1.0)),
                ("string".to_string(), Json::String("string".to_string())),
                ("boolean".to_string(), Json::Boolean(true)),
            ]
            .into_iter()
            .collect(),
        ));
        let json = json!({
            "number": 1,
            "string": "string",
            "boolean": true
        });
        assert_eq!(json, expected);
    }
}
