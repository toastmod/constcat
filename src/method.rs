
pub trait NamedTrait {
    const OBJNAME: &'static str;
}

// $obj => the object to generate strings for
// $vname => variable name for the generated string member
// $postf => the postfix to concatenate 


macro_rules! postfvar {
    ($obj:ident, $vname:ident = $postf:ident) => {
        const $postf: [u8; $obj::OBJNAME.as_bytes().len()+stringify!($postf).as_bytes().len()+1] = {
            let mut s: [u8; $obj::OBJNAME.as_bytes().len()+stringify!($postf).as_bytes().len()+1] = [0u8; $obj::OBJNAME.as_bytes().len()+stringify!($postf).as_bytes().len()+1];
            
            let mut i = 0usize;
            
            while i < s.len() {
                if i < $obj::OBJNAME.as_bytes().len() {
                    s[i] = $obj::OBJNAME.as_bytes()[i];
                }else if i >= $obj::OBJNAME.as_bytes().len() && i-$obj::OBJNAME.as_bytes().len() < stringify!($postf).as_bytes().len() {
                    s[i] = stringify!($postf).as_bytes()[i-$obj::OBJNAME.as_bytes().len()];
                }
                i+=1;
            }
            
            s
        };
    };

    ($obj:ident, $vname1:ident = $postf1:ident, $($vname:ident = $postf:ident),+) => {
        postfvar!($obj, $vname1 = $postf1);
        postfvar!($obj, $($vname = $postf),+);
    };
}

macro_rules! postftrait {
    ($obj:ident, $($vname:ident = $postf:ident),+) => {
        trait PostfixedTrait {
            postftrait!(inner1 $($vname = $postf),+);
        } 
    };

    ($obj:ident, $vname:ident = $postf:ident) => {
        trait PostfixedTrait {
            const $vname: &'static str;
        } 
    };
    
    (inner $vname1:ident = $postf1:ident) => {
        const $vname: &'static str;
    };

    (inner1 $vname1:ident = $postf1:ident,$($vname:ident = $postf:ident),+) => {
        postftrait!(inner $vname1 = $postf1);
        postftrait!(inner1 $($vname = $postf),+);
    };
}

macro_rules! postfimpl {

    // No variadics
    ($obj:ident, $vname1:ident = $postf1:ident) => {
        impl PostfixedTrait for $obj {
            postfimpl!(single $vname1 = $postf1);
        }
    };
    
    // With variadics
    ($obj:ident, $vname1:ident = $postf1:ident, $($vname:ident = $postf:ident),+) => {
        impl PostfixedTrait for $obj {
            postfimpl!(decomp $vname1 = $postf1, $($vname:ident = $postf:ident),+);
        }
    };

    (decomp $vname1:ident = $postf1:ident, $($vname:ident = $postf:ident),+) => {
        const $vname1: &'static str = {unsafe{&std::str::from_utf8_unchecked(&$postf1)}};
        postfimpl!(decomp $($vname = $postf),+);
    };

    (single $vname:ident = $postf:ident) => {
        const $vname: &'static str = {unsafe{&std::str::from_utf8_unchecked(&$postf)}};
    };
}

#[macro_export]
macro_rules! constcat {
    ($obj:ident {$($vname:ident = $postf:ident),+}) => {

        postfvar!($obj, $($vname = $postf),+);

        postftrait!($obj, $($vname = $postf),+);

        postfimpl!($obj, $($vname = $postf),+);

    };


    ($obj:ident {$vname:ident = $postf:ident}) => {

        postfvar!($obj, $vname = $postf);

        postftrait!($obj, $vname = $postf);

        postfimpl!($obj, $vname = $postf);
    
    };
}

