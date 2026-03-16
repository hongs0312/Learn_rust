mod front_of_house;

// use를 이용해 경로 단축하기
// use가 사용된 스코프에서만 유용하다
pub use crate::front_of_house::hosting; // as 별명 구문으로 새로운 이름 제공 가능

fn deliver_order() {}


mod back_of_house {
    // 구조체를 공개로 설정해도 필드값에 따라서 공개여부 설정 가능
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 열거형은 공개시 모든 배리언트 공개
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // super: 상대 경로의 시작을 자신의 부모 모듈에서 시작
        // 파일시스템에서 ..과 같은 역할
        super::deliver_order();
    }

    fn cook_order() {}
}

pub mod house {
    pub use crate::front_of_house::hosting;
}

// 만약 함수가 아래와 같이 다른 스코프 안에 있다면 무효함
// mod customer {
//     pub fn eat_at_restaurent() {}
// }
pub fn eat_at_restaurent() {
    // use키워드를 이용하면 관용적으로 함수를 호출할때 부모 모듈을 특정해야함
    hosting::add_to_waitlist();

    // pub use를 이용해 house 모듈에서 재 노출 했으므로
    // house에서 선언된 것 처럼 사용 가능
    // 실제 구조와 사용하는 사람의 예상이 다를거 같을때 사용
    // ex) a모듈에서 구현했지만 사용자는 b모듈에 있을거라고 생각할 수 있을떄
    //     b모듈에서 pub use를 써서 재노출 시키는 형식
    house::hosting::add_to_waitlist();

    // // 절대 경로
    // crate::front_of_house::hosting::add_to_waitlist();

    // // 상대 경로
    // front_of_house::hosting::add_to_waitlist();

    // 호밀(Rye) 토스트를 곁들인 여름철 조식 주문하기
    let mut meal = back_of_house::Breakfast::summer("Rye");

    //먹고 싶은 빵 바꾸기
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 다음 라인의 주석을 해제하면 컴파일되지 않음; 식사와 함께
    // 제공되는 계절 과일은 조회나 수정이 허용되지 않기 때문
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}