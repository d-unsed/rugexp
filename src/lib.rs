#[macro_use] extern crate ruru;
#[macro_use] extern crate lazy_static;

extern crate regex;

use regex::Regex;
use ruru::{AnyObject, Boolean, Class, Object, RString};

wrappable_struct! {
    Regex,
    RegexWrapper,
    REGEX_WRAPPER
}

class!(Rugexp);

methods!(
    Rugexp,
    itself,

    fn rugexp_new(pattern: RString) -> AnyObject {
        let regex = Regex::new(&pattern.unwrap().to_str()).unwrap();

        Class::from_existing("Rugexp").wrap_data(regex, &*REGEX_WRAPPER)
    }

    fn rugexp_match_q(string: RString) -> Boolean {
        let result = itself.get_data(&*REGEX_WRAPPER).is_match(string.unwrap().to_str());

        Boolean::new(result)
    }
);

pub fn define_class() {
    Class::new("Rugexp", None).define(|itself| {
        itself.def_self("new", rugexp_new);

        itself.def("match?", rugexp_match_q);
    });
}
