use bstr::{ByteSlice, CharIndices};
use core::iter::FusedIterator;
use core::str::Chars;

/// An iterator that yields a debug representation of a `Symbol` and its byte
/// contents as a sequence of `char`s.
///
/// This struct is created by the [`inspect`] method on [`Symbol`]. See its
/// documentation for more.
///
/// To format a `Symbol` directly into a writer, see [`format_inspect_into`].
///
/// [`inspect`]: crate::Symbol::inspect
/// [`Symbol`]: crate::Symbol
/// [`format_inspect_into`]: crate::Symbol::format_inspect_into
#[must_use = "Iterator"]
#[derive(Default, Debug, Clone)]
#[cfg_attr(docsrs, doc(cfg(feature = "artichoke")))]
pub struct Inspect<'a>(State<'a>);

impl<'a> From<&'a str> for Inspect<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        Self::from(value.as_bytes())
    }
}

impl<'a> From<&'a [u8]> for Inspect<'a> {
    #[inline]
    fn from(value: &'a [u8]) -> Self {
        match value {
            value if value.is_empty() => Self::default(),
            // This match arm is known to be buggy. UTF-8 well-formedness is a
            // necessary but not sufficient condition for having an unquoted
            // debug representation. The byte contents must also be a valid Ruby
            // identifier.
            //
            // See artichoke/artichoke#219.
            value if value.is_utf8() => Self(State::unquoted(value)),
            value => Self(State::quoted(value)),
        }
    }
}

impl<'a> Iterator for Inspect<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> DoubleEndedIterator for Inspect<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<'a> FusedIterator for Inspect<'a> {}

#[must_use = "Iterator"]
#[derive(Debug, Clone)]
struct State<'a> {
    string: &'a [u8],
    sigil: bool,
    open_quote: bool,
    next: [Option<Literal>; 3],
    iter: CharIndices<'a>,
    close_quote: bool,
}

impl<'a> State<'a> {
    /// Construct a `State` that will not quote byte contents after the initial
    /// `:`.
    ///
    /// This constructor produces inspect contents like `:fred`.
    #[inline]
    fn unquoted(bytes: &'a [u8]) -> Self {
        Self {
            string: bytes,
            sigil: true,
            open_quote: false,
            next: [None, None, None],
            iter: bytes.char_indices(),
            close_quote: false,
        }
    }

    /// Construct a `State` that will quote byte contents after the initial `:`.
    ///
    /// This constructor produces inspect contents like `:"Spinoso Symbol".
    #[inline]
    fn quoted(bytes: &'a [u8]) -> Self {
        Self {
            string: bytes,
            sigil: true,
            open_quote: true,
            next: [None, None, None],
            iter: bytes.char_indices(),
            close_quote: true,
        }
    }
}

impl<'a> Default for State<'a> {
    /// Construct a `State` that will render debug output for the empty slice.
    ///
    /// This constructor produces inspect contents like `:"Spinoso Symbol".
    #[inline]
    fn default() -> Self {
        Self {
            string: b"",
            sigil: true,
            open_quote: true,
            next: [None, None, None],
            iter: b"".char_indices(),
            close_quote: true,
        }
    }
}

impl<'a> Iterator for State<'a> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.sigil {
            self.sigil = false;
            return Some(':');
        }
        if self.open_quote {
            self.open_quote = false;
            return Some('"');
        }
        for cell in &mut self.next {
            let mut done = false;
            let mut next = None;
            if let Some(ref mut lit) = cell {
                next = lit.next();
                done = next.is_none();
            }
            if done {
                *cell = None;
            }
            if next.is_some() {
                return next;
            }
        }
        if let Some((start, end, ch)) = self.iter.next() {
            if ch == '\u{FFFD}' {
                let mut next = None::<char>;
                let slice = &self.string[start..end];
                let iter = slice.iter().zip(self.next.iter_mut()).enumerate();
                for (idx, (&byte, cell)) in iter {
                    let mut lit = Literal::from(byte);
                    if idx == 0 {
                        next = lit.0.next();
                    }
                    *cell = Some(lit);
                }
                return next;
            } else {
                return Some(ch);
            }
        }
        if self.close_quote {
            self.close_quote = false;
            return Some('"');
        }
        None
    }
}

impl<'a> DoubleEndedIterator for State<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.close_quote {
            self.close_quote = false;
            return Some('"');
        }
        for cell in self.next.iter_mut().rev() {
            let mut done = false;
            let mut next = None;
            if let Some(ref mut lit) = cell {
                next = lit.next_back();
                done = next.is_none();
            }
            if done {
                *cell = None;
            }
            if next.is_some() {
                return next;
            }
        }
        if let Some((start, end, ch)) = self.iter.next_back() {
            if ch == '\u{FFFD}' {
                let mut next = None::<char>;
                let slice = &self.string[start..end];
                let iter = slice.iter().zip(self.next.iter_mut()).rev().enumerate();
                for (idx, (&byte, cell)) in iter {
                    let mut lit = Literal::from(byte);
                    if idx == 0 {
                        next = lit.0.next_back();
                    }
                    *cell = Some(lit);
                }
                return next;
            } else {
                return Some(ch);
            }
        }
        if self.open_quote {
            self.open_quote = false;
            return Some('"');
        }
        if self.sigil {
            self.sigil = false;
            return Some(':');
        }
        None
    }
}

impl<'a> FusedIterator for State<'a> {}

#[must_use = "Iterator"]
#[derive(Debug, Clone)]
struct Literal(Chars<'static>);

impl From<u8> for Literal {
    /// Map from a `u8` to a String literal of a hex escape code.
    ///
    /// For example, `\xFF` or `\x0C`.
    #[allow(clippy::too_many_lines)]
    fn from(value: u8) -> Self {
        let escape = match value {
            0 => r"\x00",
            1 => r"\x01",
            2 => r"\x02",
            3 => r"\x03",
            4 => r"\x04",
            5 => r"\x05",
            6 => r"\x06",
            7 => r"\x07",
            8 => r"\x08",
            9 => r"\x09",
            10 => r"\x0A",
            11 => r"\x0B",
            12 => r"\x0C",
            13 => r"\x0D",
            14 => r"\x0E",
            15 => r"\x0F",
            16 => r"\x10",
            17 => r"\x11",
            18 => r"\x12",
            19 => r"\x13",
            20 => r"\x14",
            21 => r"\x15",
            22 => r"\x16",
            23 => r"\x17",
            24 => r"\x18",
            25 => r"\x19",
            26 => r"\x1A",
            27 => r"\x1B",
            28 => r"\x1C",
            29 => r"\x1D",
            30 => r"\x1E",
            31 => r"\x1F",
            32 => r"\x20",
            33 => r"\x21",
            34 => r"\x22",
            35 => r"\x23",
            36 => r"\x24",
            37 => r"\x25",
            38 => r"\x26",
            39 => r"\x27",
            40 => r"\x28",
            41 => r"\x29",
            42 => r"\x2A",
            43 => r"\x2B",
            44 => r"\x2C",
            45 => r"\x2D",
            46 => r"\x2E",
            47 => r"\x2F",
            48 => r"\x30",
            49 => r"\x31",
            50 => r"\x32",
            51 => r"\x33",
            52 => r"\x34",
            53 => r"\x35",
            54 => r"\x36",
            55 => r"\x37",
            56 => r"\x38",
            57 => r"\x39",
            58 => r"\x3A",
            59 => r"\x3B",
            60 => r"\x3C",
            61 => r"\x3D",
            62 => r"\x3E",
            63 => r"\x3F",
            64 => r"\x40",
            65 => r"\x41",
            66 => r"\x42",
            67 => r"\x43",
            68 => r"\x44",
            69 => r"\x45",
            70 => r"\x46",
            71 => r"\x47",
            72 => r"\x48",
            73 => r"\x49",
            74 => r"\x4A",
            75 => r"\x4B",
            76 => r"\x4C",
            77 => r"\x4D",
            78 => r"\x4E",
            79 => r"\x4F",
            80 => r"\x50",
            81 => r"\x51",
            82 => r"\x52",
            83 => r"\x53",
            84 => r"\x54",
            85 => r"\x55",
            86 => r"\x56",
            87 => r"\x57",
            88 => r"\x58",
            89 => r"\x59",
            90 => r"\x5A",
            91 => r"\x5B",
            92 => r"\x5C",
            93 => r"\x5D",
            94 => r"\x5E",
            95 => r"\x5F",
            96 => r"\x60",
            97 => r"\x61",
            98 => r"\x62",
            99 => r"\x63",
            100 => r"\x64",
            101 => r"\x65",
            102 => r"\x66",
            103 => r"\x67",
            104 => r"\x68",
            105 => r"\x69",
            106 => r"\x6A",
            107 => r"\x6B",
            108 => r"\x6C",
            109 => r"\x6D",
            110 => r"\x6E",
            111 => r"\x6F",
            112 => r"\x70",
            113 => r"\x71",
            114 => r"\x72",
            115 => r"\x73",
            116 => r"\x74",
            117 => r"\x75",
            118 => r"\x76",
            119 => r"\x77",
            120 => r"\x78",
            121 => r"\x79",
            122 => r"\x7A",
            123 => r"\x7B",
            124 => r"\x7C",
            125 => r"\x7D",
            126 => r"\x7E",
            127 => r"\x7F",
            128 => r"\x80",
            129 => r"\x81",
            130 => r"\x82",
            131 => r"\x83",
            132 => r"\x84",
            133 => r"\x85",
            134 => r"\x86",
            135 => r"\x87",
            136 => r"\x88",
            137 => r"\x89",
            138 => r"\x8A",
            139 => r"\x8B",
            140 => r"\x8C",
            141 => r"\x8D",
            142 => r"\x8E",
            143 => r"\x8F",
            144 => r"\x90",
            145 => r"\x91",
            146 => r"\x92",
            147 => r"\x93",
            148 => r"\x94",
            149 => r"\x95",
            150 => r"\x96",
            151 => r"\x97",
            152 => r"\x98",
            153 => r"\x99",
            154 => r"\x9A",
            155 => r"\x9B",
            156 => r"\x9C",
            157 => r"\x9D",
            158 => r"\x9E",
            159 => r"\x9F",
            160 => r"\xA0",
            161 => r"\xA1",
            162 => r"\xA2",
            163 => r"\xA3",
            164 => r"\xA4",
            165 => r"\xA5",
            166 => r"\xA6",
            167 => r"\xA7",
            168 => r"\xA8",
            169 => r"\xA9",
            170 => r"\xAA",
            171 => r"\xAB",
            172 => r"\xAC",
            173 => r"\xAD",
            174 => r"\xAE",
            175 => r"\xAF",
            176 => r"\xB0",
            177 => r"\xB1",
            178 => r"\xB2",
            179 => r"\xB3",
            180 => r"\xB4",
            181 => r"\xB5",
            182 => r"\xB6",
            183 => r"\xB7",
            184 => r"\xB8",
            185 => r"\xB9",
            186 => r"\xBA",
            187 => r"\xBB",
            188 => r"\xBC",
            189 => r"\xBD",
            190 => r"\xBE",
            191 => r"\xBF",
            192 => r"\xC0",
            193 => r"\xC1",
            194 => r"\xC2",
            195 => r"\xC3",
            196 => r"\xC4",
            197 => r"\xC5",
            198 => r"\xC6",
            199 => r"\xC7",
            200 => r"\xC8",
            201 => r"\xC9",
            202 => r"\xCA",
            203 => r"\xCB",
            204 => r"\xCC",
            205 => r"\xCD",
            206 => r"\xCE",
            207 => r"\xCF",
            208 => r"\xD0",
            209 => r"\xD1",
            210 => r"\xD2",
            211 => r"\xD3",
            212 => r"\xD4",
            213 => r"\xD5",
            214 => r"\xD6",
            215 => r"\xD7",
            216 => r"\xD8",
            217 => r"\xD9",
            218 => r"\xDA",
            219 => r"\xDB",
            220 => r"\xDC",
            221 => r"\xDD",
            222 => r"\xDE",
            223 => r"\xDF",
            224 => r"\xE0",
            225 => r"\xE1",
            226 => r"\xE2",
            227 => r"\xE3",
            228 => r"\xE4",
            229 => r"\xE5",
            230 => r"\xE6",
            231 => r"\xE7",
            232 => r"\xE8",
            233 => r"\xE9",
            234 => r"\xEA",
            235 => r"\xEB",
            236 => r"\xEC",
            237 => r"\xED",
            238 => r"\xEE",
            239 => r"\xEF",
            240 => r"\xF0",
            241 => r"\xF1",
            242 => r"\xF2",
            243 => r"\xF3",
            244 => r"\xF4",
            245 => r"\xF5",
            246 => r"\xF6",
            247 => r"\xF7",
            248 => r"\xF8",
            249 => r"\xF9",
            250 => r"\xFA",
            251 => r"\xFB",
            252 => r"\xFC",
            253 => r"\xFD",
            254 => r"\xFE",
            255 => r"\xFF",
        };
        Self(escape.chars())
    }
}

impl Iterator for Literal {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n)
    }

    #[inline]
    fn count(self) -> usize {
        self.0.count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.0.last()
    }
}

impl DoubleEndedIterator for Literal {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n)
    }
}

impl FusedIterator for Literal {}

#[cfg(test)]
mod tests {
    use super::Inspect;
    use alloc::string::String;

    // From spec/core/symbol/inspect_spec.rb:
    //
    // ```ruby
    // symbols = {
    //   fred:         ":fred",
    //   :fred?     => ":fred?",
    //   :fred!     => ":fred!",
    //   :$ruby     => ":$ruby",
    //   :@ruby     => ":@ruby",
    //   :@@ruby    => ":@@ruby",
    //   :"$ruby!"  => ":\"$ruby!\"",
    //   :"$ruby?"  => ":\"$ruby?\"",
    //   :"@ruby!"  => ":\"@ruby!\"",
    //   :"@ruby?"  => ":\"@ruby?\"",
    //   :"@@ruby!" => ":\"@@ruby!\"",
    //   :"@@ruby?" => ":\"@@ruby?\"",
    //
    //   :$-w       => ":$-w",
    //   :"$-ww"    => ":\"$-ww\"",
    //   :"$+"      => ":$+",
    //   :"$~"      => ":$~",
    //   :"$:"      => ":$:",
    //   :"$?"      => ":$?",
    //   :"$<"      => ":$<",
    //   :"$_"      => ":$_",
    //   :"$/"      => ":$/",
    //   :"$'"      => ":$'",
    //   :"$\""     => ":$\"",
    //   :"$$"      => ":$$",
    //   :"$."      => ":$.",
    //   :"$,"      => ":$,",
    //   :"$`"      => ":$`",
    //   :"$!"      => ":$!",
    //   :"$;"      => ":$;",
    //   :"$\\"     => ":$\\",
    //   :"$="      => ":$=",
    //   :"$*"      => ":$*",
    //   :"$>"      => ":$>",
    //   :"$&"      => ":$&",
    //   :"$@"      => ":$@",
    //   :"$1234"   => ":$1234",
    //
    //   :-@        => ":-@",
    //   :+@        => ":+@",
    //   :%         => ":%",
    //   :&         => ":&",
    //   :*         => ":*",
    //   :**        => ":**",
    //   :"/"       => ":/",     # lhs quoted for emacs happiness
    //   :<         => ":<",
    //   :<=        => ":<=",
    //   :<=>       => ":<=>",
    //   :==        => ":==",
    //   :===       => ":===",
    //   :=~        => ":=~",
    //   :>         => ":>",
    //   :>=        => ":>=",
    //   :>>        => ":>>",
    //   :[]        => ":[]",
    //   :[]=       => ":[]=",
    //   :"\<\<"    => ":\<\<",
    //   :^         => ":^",
    //   :"`"       => ":`",     # for emacs, and justice!
    //   :~         => ":~",
    //   :|         => ":|",
    //
    //   :"!"       => [":\"!\"",  ":!" ],
    //   :"!="      => [":\"!=\"", ":!="],
    //   :"!~"      => [":\"!~\"", ":!~"],
    //   :"\$"      => ":\"$\"", # for justice!
    //   :"&&"      => ":\"&&\"",
    //   :"'"       => ":\"\'\"",
    //   :","       => ":\",\"",
    //   :"."       => ":\".\"",
    //   :".."      => ":\"..\"",
    //   :"..."     => ":\"...\"",
    //   :":"       => ":\":\"",
    //   :"::"      => ":\"::\"",
    //   :";"       => ":\";\"",
    //   :"="       => ":\"=\"",
    //   :"=>"      => ":\"=>\"",
    //   :"\?"      => ":\"?\"", # rawr!
    //   :"@"       => ":\"@\"",
    //   :"||"      => ":\"||\"",
    //   :"|||"     => ":\"|||\"",
    //   :"++"      => ":\"++\"",
    //
    //   :"\""      => ":\"\\\"\"",
    //   :"\"\""    => ":\"\\\"\\\"\"",
    //
    //   :"9"       => ":\"9\"",
    //   :"foo bar" => ":\"foo bar\"",
    //   :"*foo"    => ":\"*foo\"",
    //   :"foo "    => ":\"foo \"",
    //   :" foo"    => ":\" foo\"",
    //   :" "       => ":\" \"",
    // }
    // ```

    #[test]
    fn empty() {
        let inspect = Inspect::from("");
        let debug = inspect.collect::<String>();
        assert_eq!(debug, r#":"""#);
    }

    #[test]
    fn empty_backwards() {
        let mut inspect = Inspect::from("");
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), Some(':'));
        assert_eq!(inspect.next_back(), None);
        assert_eq!(inspect.next(), None);

        let mut inspect = Inspect::from("");
        assert_eq!(inspect.next(), Some(':'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), None);
        assert_eq!(inspect.next(), None);

        let mut inspect = Inspect::from("");
        assert_eq!(inspect.next(), Some(':'));
        assert_eq!(inspect.next(), Some('"'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), None);
        assert_eq!(inspect.next(), None);

        let mut inspect = Inspect::from("");
        assert_eq!(inspect.next(), Some(':'));
        assert_eq!(inspect.next(), Some('"'));
        assert_eq!(inspect.next(), Some('"'));
        assert_eq!(inspect.next_back(), None);
        assert_eq!(inspect.next(), None);
    }

    #[test]
    fn fred() {
        let inspect = Inspect::from("fred");
        let debug = inspect.collect::<String>();
        assert_eq!(debug, ":fred");
    }

    #[test]
    fn fred_backwards() {
        let mut inspect = Inspect::from("fred");
        assert_eq!(inspect.next_back(), Some('d'));
        assert_eq!(inspect.next_back(), Some('e'));
        assert_eq!(inspect.next_back(), Some('r'));
        assert_eq!(inspect.next_back(), Some('f'));
        assert_eq!(inspect.next_back(), Some(':'));
        assert_eq!(inspect.next_back(), None);
        assert_eq!(inspect.next(), None);
    }

    #[test]
    fn invalid_utf8() {
        let inspect = Inspect::from(&b"invalid-\xFF-utf8"[..]);
        let debug = inspect.collect::<String>();
        assert_eq!(debug, r#":"invalid-\xFF-utf8""#);
    }

    #[test]
    fn invalid_utf8_backwards() {
        let mut inspect = Inspect::from(&b"invalid-\xFF-utf8"[..]);
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), Some('8'));
        assert_eq!(inspect.next_back(), Some('f'));
        assert_eq!(inspect.next_back(), Some('t'));
        assert_eq!(inspect.next_back(), Some('u'));
        assert_eq!(inspect.next_back(), Some('-'));
        assert_eq!(inspect.next_back(), Some('F'));
        assert_eq!(inspect.next_back(), Some('F'));
        assert_eq!(inspect.next_back(), Some('x'));
        assert_eq!(inspect.next_back(), Some('\\'));
        assert_eq!(inspect.next_back(), Some('-'));
        assert_eq!(inspect.next_back(), Some('d'));
        assert_eq!(inspect.next_back(), Some('i'));
        assert_eq!(inspect.next_back(), Some('l'));
        assert_eq!(inspect.next_back(), Some('a'));
        assert_eq!(inspect.next_back(), Some('v'));
        assert_eq!(inspect.next_back(), Some('n'));
        assert_eq!(inspect.next_back(), Some('i'));
        assert_eq!(inspect.next_back(), Some('"'));
        assert_eq!(inspect.next_back(), Some(':'));
        assert_eq!(inspect.next_back(), None);
        assert_eq!(inspect.next(), None);
    }
}
