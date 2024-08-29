#[macro_export]
macro_rules! constcat {
    ($name:ident, $postfix:ident) => {
        const {unsafe{&std::str::from_utf8_unchecked(&const {
            let mut s: [u8; $name.as_bytes().len()+$postfix.as_bytes().len()+1] = [0u8; $name.as_bytes().len()+$postfix.as_bytes().len()+1];

            let mut i = 0usize;

            while i < s.len() {
                if i < $name.as_bytes().len() {
                    s[i] = $name.as_bytes()[i];
                }else if i >= $name.as_bytes().len() && i-$name.as_bytes().len() < $postfix.as_bytes().len() {
                    s[i] = $postfix.as_bytes()[i-$name.as_bytes().len()];
                }
                i+=1;
            }

            s 
        })}};
    };
}

mod test {
    const name: &str = "MyName🐱";
    const postfix: &str = "_type";
    const totally_safe_name_trust_me_im_a_cat: &'static str = constcat!(name, postfix); 

    #[test]
    fn it_works() {
        println!("Hello, world! Looks like my cat's name is {}", totally_safe_name_trust_me_im_a_cat);
    } 
}