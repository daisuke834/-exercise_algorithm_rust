pub fn hogefunc(i: i32) {
    match fugafunc(i) {
        Ok(code) => println!("{} -> Success, code = {}", i, code),
        Err(err) => println!("{} -> Err: message = {}", i, err),
    };
}

fn fugafunc(i: i32) -> Result<i32, String> {
    println!("Hello, module!");
    if i > 0 {
        Ok(777)
    } else {
        Err(String::from("dame"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fugafunc() {
        assert_eq!(fugafunc(2), Ok(777));
        assert_eq!(fugafunc(-1), Err(String::from("dame")));
    }
}
