pub mod mruby;
pub mod trampoline;

#[derive(Debug)]
pub struct String;

// Tests from String core docs in Ruby 2.6.3
// https://ruby-doc.org/core-2.6.3/String.html
#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn string_equal_squiggle() {
        let mut interp = crate::interpreter().expect("init");

        let value = interp.eval(br#""cat o' 9 tails" =~ /\d/"#).unwrap();
        let value = value.try_into::<Option<i64>>().unwrap();
        assert_eq!(value, Some(7));
        let value = interp.eval(br#""cat o' 9 tails" =~ 9"#).unwrap();
        let value = value.try_into::<Option<i64>>().unwrap();
        assert!(value.is_none());
    }

    #[test]
    fn string_idx() {
        let mut interp = crate::interpreter().expect("init");

        let result = interp
            .eval(br"'hello there'[/[aeiou](.)\1/]")
            .unwrap()
            .try_into::<&str>()
            .unwrap();
        assert_eq!(result, "ell");
        let result = interp
            .eval(br"'hello there'[/[aeiou](.)\1/, 0]")
            .unwrap()
            .try_into::<&str>()
            .unwrap();
        assert_eq!(result, "ell");
        let result = interp
            .eval(br"'hello there'[/[aeiou](.)\1/, 1]")
            .unwrap()
            .try_into::<&str>()
            .unwrap();
        assert_eq!(result, "l");
        let result = interp
            .eval(br"'hello there'[/[aeiou](.)\1/, 2]")
            .unwrap()
            .try_into::<Option<&str>>()
            .unwrap();
        assert_eq!(result, None);
        let result = interp
            .eval(br"'hello there'[/(?<vowel>[aeiou])(?<non_vowel>[^aeiou])/, 'non_vowel']")
            .unwrap()
            .try_into::<&str>()
            .unwrap();
        assert_eq!(result, "l");
        let result = interp
            .eval(br"'hello there'[/(?<vowel>[aeiou])(?<non_vowel>[^aeiou])/, 'vowel']")
            .unwrap()
            .try_into::<&str>()
            .unwrap();
        assert_eq!(result, "e");
    }

    #[test]
    fn string_scan() {
        let mut interp = crate::interpreter().expect("init");

        let s = interp.convert_mut("abababa");
        let result = s
            .funcall::<Vec<&str>>("scan", &[interp.eval(b"/./").unwrap()], None)
            .unwrap();
        assert_eq!(result, vec!["a", "b", "a", "b", "a", "b", "a"]);
        let result = s
            .funcall::<Vec<&str>>("scan", &[interp.eval(b"/../").unwrap()], None)
            .unwrap();
        assert_eq!(result, vec!["ab", "ab", "ab"]);
        let result = s
            .funcall::<Vec<&str>>("scan", &[interp.convert_mut("aba")], None)
            .unwrap();
        assert_eq!(result, vec!["aba", "aba"]);
        let result = s
            .funcall::<Vec<&str>>("scan", &[interp.convert_mut("no no no")], None)
            .unwrap();
        assert_eq!(result, Vec::<&str>::new());
    }

    #[test]
    fn string_unary_minus() {
        let mut interp = crate::interpreter().expect("init");

        let s = interp.eval(b"-'abababa'").expect("eval");
        let result = s.funcall::<bool>("frozen?", &[], None).unwrap();
        assert!(result);
        let result = s.funcall::<&str>("itself", &[], None).unwrap();
        assert_eq!(result, "abababa");
    }
}
