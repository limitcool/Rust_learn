fn first_word() {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..1];
        }
    }

    &s[..]
}
fn second_word(s: &String) ->&str {
    
}