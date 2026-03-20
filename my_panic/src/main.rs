use std::fs::File;
use std::io::{self, ErrorKind, Read, Error};

fn main() {
    // 명시적으로 패닉 선언 가능
    // panic!("crash and burn");
    
    // RUST_TRACEBACK=1 환경변수 설정을 통해 실행된 라인과 실핼할 라인을 확인 가능
    // let v = [1, 2, 3];
    // v[99];

    // File::open 의 반환값은 Result<T, E>
    // T에는 파일 읽기에 성공한 std::fs::File, E에는 std::io::Error 타입
    let greeting_file_result = File::open("hello.txt");

    // match 표현식을 이용해 결과에 따라 처리하기
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        },
    };

    // 에러 발생시 unwrap()나 expect()으로 간단히 처리 가능
    let greeting_file = File::open("hello.txt").unwrap(); // .expect("에러 메시지")로 painc! 에러메시지를 선택할 수도 있음

    

}
// 에러 전파하기 error propagation
fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // ?연산자를 이용해 숏컷 이용 가능 (위 주석 코드와 동일)
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}
