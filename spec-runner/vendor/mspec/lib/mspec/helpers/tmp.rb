# Creates a temporary directory in the current working directory
# for temporary files created while running the specs. All specs
# should clean up any temporary files created so that the temp
# directory is empty when the process exits.

# mruby does not support `at_exit` and there is no need to clean up so only set
# the `SPEC_TEMP_DIR` constants and return.
SPEC_TEMP_DIR_PID = "999"
SPEC_TEMP_DIR = "rubyspec_temp"
SPEC_TEMP_UNIQUIFIER = "0"
return

SPEC_TEMP_DIR_PID = Process.pid

if spec_temp_dir = ENV["SPEC_TEMP_DIR"]
  spec_temp_dir = File.realdirpath(spec_temp_dir)
else
  spec_temp_dir = "#{File.realpath(Dir.pwd)}/rubyspec_temp/#{SPEC_TEMP_DIR_PID}"
end
SPEC_TEMP_DIR = spec_temp_dir

SPEC_TEMP_UNIQUIFIER = "0"

at_exit do
  begin
    if SPEC_TEMP_DIR_PID == Process.pid
      Dir.delete SPEC_TEMP_DIR if File.directory? SPEC_TEMP_DIR
    end
  rescue SystemCallError
    STDERR.puts <<-EOM

-----------------------------------------------------
The rubyspec temp directory is not empty. Ensure that
all specs are cleaning up temporary files:
  #{SPEC_TEMP_DIR}
-----------------------------------------------------

    EOM
  rescue Object => e
    STDERR.puts "failed to remove spec temp directory"
    STDERR.puts e.message
  end
end

def tmp(name, uniquify = true)
  mkdir_p SPEC_TEMP_DIR unless Dir.exist? SPEC_TEMP_DIR

  if uniquify and !name.empty?
    slash = name.rindex "/"
    index = slash ? slash + 1 : 0
    name.insert index, "#{SPEC_TEMP_UNIQUIFIER.succ!}-"
  end

  File.join SPEC_TEMP_DIR, name
end
