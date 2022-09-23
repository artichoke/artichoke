#!/usr/bin/env ruby
# frozen_string_literal: true

require 'fileutils'

def render(exc:, type:)
  <<~AUTOGEN
    // @generated

    use alloc::borrow::Cow;
    use alloc::string::String;
    use alloc::vec::Vec;
    use core::fmt;
    #[cfg(feature = "std")]
    use std::error;

    use scolapasta_string_escape::format_debug_escape_into;

    use crate::RubyException;

    /// Ruby `#{exc}` error type.
    ///
    /// Descendants of class [`Exception`] are used to communicate between
    /// [`Kernel#raise`] and `rescue` statements in `begin ... end` blocks.
    /// Exception objects carry information about the exception – its type (the
    /// exception's class name), an optional descriptive string, and optional
    /// traceback information. `Exception` subclasses may add additional information
    /// like [`NameError#name`].
    ///
    /// [`Exception`]: https://ruby-doc.org/core-3.1.2/Exception.html
    /// [`Kernel#raise`]: https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-raise
    /// [`NameError#name`]: https://ruby-doc.org/core-3.1.2/NameError.html#method-i-name
    #[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct #{type} {
        message: Cow<'static, [u8]>,
    }

    impl #{type} {
        /// Construct a new, default `#{exc}` Ruby exception.
        ///
        /// This constructor sets the exception message to `#{exc}`.
        ///
        /// # Examples
        ///
        /// ```
        /// # use spinoso_exception::*;
        /// let exception = #{type}::new();
        /// assert_eq!(exception.message(), b"#{exc}");
        /// ```
        #[inline]
        #[must_use]
        pub const fn new() -> Self {
            const DEFAULT_MESSAGE: &[u8] = b"#{exc}";

            // `Exception` objects initialized via (for example)
            // `raise RuntimeError` or `RuntimeError.new` have `message`
            // equal to the exception's class name.
            let message = Cow::Borrowed(DEFAULT_MESSAGE);
            Self { message }
        }

        /// Construct a new, `#{exc}` Ruby exception with the given
        /// message.
        ///
        /// # Examples
        ///
        /// ```
        /// # use spinoso_exception::*;
        /// let exception = #{type}::with_message("an error occurred");
        /// assert_eq!(exception.message(), b"an error occurred");
        /// ```
        #[inline]
        #[must_use]
        pub const fn with_message(message: &'static str) -> Self {
            let message = Cow::Borrowed(message.as_bytes());
            Self { message }
        }

        /// Return the message this Ruby exception was constructed with.
        ///
        /// # Examples
        ///
        /// ```
        /// # use spinoso_exception::*;
        /// let exception = #{type}::new();
        /// assert_eq!(exception.message(), b"#{exc}");
        /// let exception = #{type}::from("something went wrong");
        /// assert_eq!(exception.message(), b"something went wrong");
        /// ```
        #[inline]
        #[must_use]
        pub fn message(&self) -> &[u8] {
            self.message.as_ref()
        }

        /// Return this Ruby exception's class name.
        ///
        /// # Examples
        ///
        /// ```
        /// # use spinoso_exception::*;
        /// let exception = #{type}::new();
        /// assert_eq!(exception.name(), "#{exc}");
        /// ```
        #[inline]
        #[must_use]
        pub const fn name(&self) -> &'static str {
            "#{exc}"
        }
    }

    impl From<String> for #{type} {
        #[inline]
        fn from(message: String) -> Self {
            let message = Cow::Owned(message.into_bytes());
            Self { message }
        }
    }

    impl From<&'static str> for #{type} {
        #[inline]
        fn from(message: &'static str) -> Self {
            let message = Cow::Borrowed(message.as_bytes());
            Self { message }
        }
    }

    impl From<Cow<'static, str>> for #{type} {
        #[inline]
        fn from(message: Cow<'static, str>) -> Self {
            let message = match message {
                Cow::Borrowed(s) => Cow::Borrowed(s.as_bytes()),
                Cow::Owned(s) => Cow::Owned(s.into_bytes()),
            };
            Self { message }
        }
    }

    impl From<Vec<u8>> for #{type} {
        #[inline]
        fn from(message: Vec<u8>) -> Self {
            let message = Cow::Owned(message);
            Self { message }
        }
    }

    impl From<&'static [u8]> for #{type} {
        #[inline]
        fn from(message: &'static [u8]) -> Self {
            let message = Cow::Borrowed(message);
            Self { message }
        }
    }

    impl From<Cow<'static, [u8]>> for #{type} {
        #[inline]
        fn from(message: Cow<'static, [u8]>) -> Self {
            Self { message }
        }
    }

    impl fmt::Display for #{type} {
        #[inline]
        fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.name())?;
            f.write_str(" (")?;
            let message = self.message.as_ref();
            format_debug_escape_into(&mut f, message)?;
            f.write_str(")")?;
            Ok(())
        }
    }

    #[cfg(feature = "std")]
    impl error::Error for #{type} {}

    impl RubyException for #{type} {
        #[inline]
        fn message(&self) -> Cow<'_, [u8]> {
            Cow::Borrowed(Self::message(self))
        }

        #[inline]
        fn name(&self) -> Cow<'_, str> {
            Cow::Borrowed(Self::name(self))
        }
    }
  AUTOGEN
end

exception_classes = [
  { type_name: 'Exception' },
  { type_name: 'NoMemoryError' },
  { type_name: 'ScriptError' },
  { type_name: 'LoadError' },
  { type_name: 'NotImplementedError' },
  { type_name: 'SyntaxError' },
  { type_name: 'SecurityError' },
  { type_name: 'SignalException' },
  { type_name: 'Interrupt' },
  # `StandardError` is the default for `rescue`.
  { type_name: 'StandardError' },
  { type_name: 'ArgumentError' },
  { type_name: 'UncaughtThrowError' },
  { type_name: 'EncodingError' },
  { type_name: 'FiberError' },
  { type_name: 'IOError' },
  { type_name: 'EOFError' },
  { type_name: 'IndexError' },
  { type_name: 'KeyError' },
  { type_name: 'StopIteration' },
  { type_name: 'LocalJumpError' },
  { type_name: 'NameError' },
  { type_name: 'NoMethodError' },
  { type_name: 'RangeError' },
  { type_name: 'FloatDomainError' },
  { type_name: 'RegexpError' },
  # `RuntimeError` is the default for `raise`.
  { type_name: 'RuntimeError' },
  { type_name: 'FrozenError' },
  { type_name: 'SystemCallError' },
  # The `Errno` module contains dynamically created `Exception` subclasses when
  # unencounted syscall errno are encountered.
  #
  # Artichoke does not support the `Errno` module.
  # { type_name: 'Errno::*)' },
  { type_name: 'ThreadError' },
  { type_name: 'TypeError' },
  { type_name: 'ZeroDivisionError' },
  { type_name: 'SystemExit' },
  { type_name: 'SystemStackError' },
  # Fatal interpreter error. Impossible to rescue.
  { type_name: 'Fatal', class_name: 'fatal' }
]

FileUtils.mkdir_p(File.join(__dir__, '..', 'src', 'core'))

filename = File.join(__dir__, '..', 'src', 'core', 'mod.rs')

File.open(filename, 'w') do |rs|
  rs.puts '// @generated'
  rs.puts ''
  rs.puts '//! Ruby exception class implementations.'
  rs.puts '//!'
  rs.puts '//! See crate-level documentation for more about the types exposed in this'
  rs.puts '//! module.'
  rs.puts

  configs = exception_classes.sort_by do |config|
    config.fetch(:type_name).downcase
  end
  configs.each do |config|
    type = config.fetch(:type_name)
    mod = type.downcase
    rs.puts "mod #{mod};"
  end
  rs.puts
  configs.each do |config|
    type = config.fetch(:type_name)
    mod = type.downcase
    rs.puts "pub use #{mod}::#{type};"
  end
end

exception_classes.each do |config|
  type = config.fetch(:type_name)
  name = config.fetch(:class_name, type)

  filename = File.join(__dir__, '..', 'src', 'core', "#{name.downcase}.rs")

  rendered = render(exc: name, type: type)
  File.write(filename, rendered)
end
