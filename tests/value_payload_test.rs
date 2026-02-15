use rustling::values::{Value, ValueKind};
use rustling_core::{NodePayload, StashIndexable};

#[test]
fn test_value_implements_node_payload() {
    let value = Value::Integer(42);

    // 测试 extract_payload() 方法
    assert_eq!(value.extract_payload(), Some(value.clone()));
}

#[test]
fn test_value_implements_stash_indexable() {
    let value = Value::Integer(42);

    // 测试 index() 方法 - 应该返回 ValueKind::Integer
    assert_eq!(value.index(), ValueKind::Integer);

    // 测试其他类型
    assert_eq!(Value::Float(3.14).index(), ValueKind::Float);
}

#[test]
fn test_value_clone() {
    let value1 = Value::Integer(42);
    let value2 = value1.clone();

    assert_eq!(value1, value2);
}
