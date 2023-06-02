pub fn add(left: usize, right: usize) -> usize {
    left + right
}
//use cargo test run the following tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4,"this is a custom error message");
    }

    #[test]
    #[should_panic(expected = "this is a custom error message")]
    fn it_fails() {
        let result = add(2, 2);
        assert_eq!(result, 5,"this is a custom error message");
    }

    /*
    use Result as the return value
    return Ok(result) test successful
    return Err(error) test failed
    */
    #[test]
    #[ignore]
    //run the ignore test: cargo test -- --ignored
    fn it_works2() -> Result<(),String>{
        if add(2, 2) == 5 {
            Ok(())
        } else {
            Err(String::from("this is a custom error message"))
        }
    }
}
/*
触发测试失败：
每个测试都在一个独立的线程
触发panic则测试失败

assert!(true) 如果assert!(false)则调用panic！，测试失败

assert_eq!()断言失败：调用panic!，并且打印其参数（用debug打印，所以比较值必须实现PartialEq和Debug Trait）
assert_ne!() 这两个Trait都是派生Trait，通常可以直接在结构体或枚举上添加 #[derive(PartialEq, Debug)] 注解

assert_matches!()
assert_not_matches!()
*/

/*
    cargo test -- --test-threads=1
    cargo test -- --show-output
    cargo test (test name)
*/

/*
单元测试，可以测试private function
集成测试
*/