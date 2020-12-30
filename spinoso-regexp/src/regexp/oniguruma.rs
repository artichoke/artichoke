use onig::RegexOptions;

use crate::Options;

impl From<Options> for RegexOptions {
    fn from(opts: Options) -> Self {
        let mut bits = RegexOptions::REGEX_OPTION_NONE;

        bits.set(RegexOptions::REGEX_OPTION_MULTILINE, opts.multiline().is_enabled());
        bits.set(RegexOptions::REGEX_OPTION_IGNORECASE, opts.ignore_case().is_enabled());
        bits.set(RegexOptions::REGEX_OPTION_EXTEND, opts.extended().is_enabled());

        bits
    }
}
