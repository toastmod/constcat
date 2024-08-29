// macro_rules! constcat {
//     (name:ident, postfix:ident, $c:token) => {
        
//     };
// }

const name: &str = "MyName🐱";
const postfix: &str = "_type";

const fn concat() -> [u8; name.as_bytes().len()+postfix.as_bytes().len()+1] {
    let mut s: [u8; name.as_bytes().len()+postfix.as_bytes().len()+1] = [0u8; name.as_bytes().len()+postfix.as_bytes().len()+1];

    let mut i = 0usize;

    while i < s.len() {
        if i < name.as_bytes().len() {
            s[i] = name.as_bytes()[i];
        }else if i >= name.as_bytes().len() && i-name.as_bytes().len() < postfix.as_bytes().len() {
            s[i] = postfix.as_bytes()[i-name.as_bytes().len()];
        }
        i+=1;
    }

    s
}

const totally_safe_name_trust_me_im_a_cat: &'static str = {unsafe{&std::str::from_utf8_unchecked(&concat())}};


pub fn test_this() {
    println!("Hello, world! Looks like my cat's name is {}", totally_safe_name_trust_me_im_a_cat);
}