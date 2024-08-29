macro_rules! variprac {

    ($name:ident) => {
        const $name: usize = 0usize;
    };

    ($name:ident, $($name2:ident),+) => {
        variprac!($name);
        variprac!($($name2),+);
    };
}

variprac!(dummy, dumdum, dummmer);

const lol: usize = dummy + dumdum+ dummmer;