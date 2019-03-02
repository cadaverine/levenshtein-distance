use super::get_levenshtein_distance;

#[test]
fn test_1() {
    assert_eq!(get_levenshtein_distance("овал", "кровля"), 4);
}

#[test]
fn test_2() {
    assert_eq!(get_levenshtein_distance("добро", "бобры"), 2);
}

#[test]
fn test_3() {
    assert_eq!(
        get_levenshtein_distance("абстракция", "аннотация"),
        5
    );
}

#[test]
fn test_4() {
    assert_eq!(get_levenshtein_distance("", ""), 0);
}

#[test]
fn test_5() {
    assert_eq!(
        get_levenshtein_distance(
            "определенно длинный текст",
            "текст достаточно длинный"
        ),
        19
    );
}
