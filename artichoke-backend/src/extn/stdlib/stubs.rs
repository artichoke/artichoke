use crate::load::LoadSources;
use crate::Artichoke;
use crate::ArtichokeError;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp.def_rb_source_file("erb.rb", "class ERB; def initialize(*args); end; end")?;
    interp.def_rb_source_file("time.rb", "")?;
    interp.def_rb_source_file("fileutils.rb", "")?;
    interp.def_rb_source_file("tempfile.rb", "")?;
    interp.def_rb_source_file("openssl.rb", "")?;
    interp.def_rb_source_file("zlib.rb", "")?;
    interp.def_rb_source_file(
        "securerandom.rb",
        "class SecureRandom; def self.hex(*args); '87694e9e5231abca6de39c58cdfbe307'; end; def self.uuid; 'fb70d164-031c-4616-aeb4-41a31295fa5b'; end; end",
    )?;
    interp.def_rb_source_file("digest.rb", "require 'digest/sha1'")?;
    interp.def_rb_source_file("digest/sha1.rb", "module Digest; class SHA1; def self.hexdigest(*args); 'a9993e364706816aba3e25717850c26c9cd0d89d'; end; def self.base64digest(*args); 'qZk+NkcGgWq6PiVxeFDCbJzQ2J0='; end; end; end")?;
    interp.def_rb_source_file("base64.rb", "")?;
    interp.def_rb_source_file("logger.rb", "")?;
    Ok(())
}
