#!/usr/bin/env ruby

require 'json'

compile_commands = File.open('compile_commands.json') do |f|
  JSON.parse(f.read)
end

compile_commands = compile_commands.select do |ccmd|
  ccmd['arguments'].include?('-DMRB_DISABLE_STDIO')
end

compile_commands = compile_commands.reject do |ccmd|
  ccmd['file'] == 'src/vm.c'
end

compile_commands = compile_commands.reject do |ccmd|
  ccmd['file'].include?('mrbgems')
end

File.open('compile_commands.sys.json', 'w') do |f|
  f.write(JSON.pretty_generate(compile_commands))
end

exec('c2rust transpile --verbose --output-dir . --emit-build-files compile_commands.sys.json -- -isystem /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include')
