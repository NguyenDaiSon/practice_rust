/// This program is to learn basic about lifetime
fn main() {
    let a1 = YesNoAnswer::Yes;
    let a2 = YesNoAnswer::No;
    let survey = SurveyForm { q1: &a1, q2: &a2 };
    println!("{:?}", survey);
}

#[derive(Debug)]
enum YesNoAnswer {
    Yes,
    No,
}

#[derive(Debug)]
struct SurveyForm<'a> {
    q1: &'a YesNoAnswer,
    q2: &'a YesNoAnswer,
}
