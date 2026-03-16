use crate::garden::vegetables::Asparagus;

pub mod garden; // 1번으로 확인 (중괄호안 인라인)

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}