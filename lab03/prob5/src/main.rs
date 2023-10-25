#[derive(Debug)]

enum OpinionError {
    PureHate
}

fn favorite_case_style () -> Option<String> {
    Some(String::from("camelCase"))
}

fn appreciation_for_snake_case () -> Option<u8> {
    None
}

fn appreciation_for_not_putting_the_opening_bracket_on_a_new_line () -> Option<u8> {
    None
}

fn his_opinion_on_rust () -> Result<String, OpinionError> {
    let fav_case: String;
    match favorite_case_style() {
        Some(f) => fav_case = String::from(f),
        None => fav_case = String::from("None in particular.")
    }
    let a1: u8;
    match appreciation_for_snake_case() {
        Some(x) => a1 = x,
        None => a1=0
    }
    let a2: u8;
    match appreciation_for_not_putting_the_opening_bracket_on_a_new_line() {
        Some(x) => a2 = x,
        None => a2=0 
    }
    if a1+a2 < 1 {
        return Err(OpinionError::PureHate);
    }

    let grade: f32 = (a1+a2)as f32/2 as f32;
    let mut answer: String = String::from("");
    answer += "He prefers putting '{' on a newline and ";
    answer += &fav_case;
    answer += ", so he gives it a ";
    answer += &grade.to_string();
    answer += "/10.";
    Ok(answer)

}

fn main() {
    match his_opinion_on_rust() {
        Ok(s) => println!("{s}"),
        Err(e) => println!("Wow, he hates Rust (especially the text formatting) so much, the code threw error '{:?}'!", e)
    }
}
