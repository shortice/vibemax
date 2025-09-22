use vibemax::vibemax;


#[test]
fn one() {
    let result = vibemax(vec![2, 11, 4]);
    assert_eq!(result, 11);
}

#[test]
fn two() {
    let result = vibemax(vec![2, 2]);
    assert_eq!(result, 2);
}

#[test]
fn three() {
    let result = vibemax(vec![10, 100, -100, 500]);
    assert_eq!(result, 500);
}
