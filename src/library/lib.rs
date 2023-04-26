pub fn my_split(input: String) -> Vec<String> {
    let mut result = vec![];
    let mut current_word = String::new();

    for c in input.chars() {
        match c {
            ',' => {
                result.push(current_word.clone());
                current_word.clear();
                result.push(c.to_string())
            }
            ' ' => {
                result.push(current_word.clone());
                current_word.clear();
            }
            '\n' => {
                result.push(current_word.clone());
                current_word.clear();
            }
            _ => current_word.push(c),
        }
    }

    if !current_word.is_empty() {
        result.push(current_word)
    }

    result
}

pub fn create_vec_from_string(file_contents: String) -> Vec<String> {
    let content: Vec<String> = my_split(file_contents);
    content.into_iter().filter(|c| !c.is_empty()).collect()
}
