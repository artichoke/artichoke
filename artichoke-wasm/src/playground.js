import "bootstrap";
import "bootstrap/dist/css/bootstrap.min.css";
import ace from "ace-builds";
import "ace-builds/webpack-resolver";
import "artichoke-wasm/artichoke_wasm.wasm";
import "artichoke-wasm/deps/artichoke-wasm.js";

const sample = `
# The following calls to Kernel#require include real implementations of Ruby
# Standard Library packages.
#
# https://ruby-doc.org/stdlib-2.5.1/libdoc/forwardable/rdoc/Forwardable.html
# https://ruby-doc.org/stdlib-2.6.3/libdoc/set/rdoc/Set.html
require 'forwardable'
require 'set'

class Registry
  extend Forwardable
  def_delegators :@records, :add, :to_a

  def initialize
    @records = Set.new
  end
end

registry = Registry.new

10.times do |record|
  registry.add("Artichoke")
  registry.add("💎")
end

puts registry.to_a
registry.to_a
`;

ace.edit("editor", {
  mode: "ace/mode/ruby",
  theme: "ace/theme/monokai",
  fontSize: 14,
  tabSize: 2,
  useSoftTabs: true
});

ace.edit("editor").setValue(sample.trim(), -1);

const read_string = (state, ptr) => {
  const len = window._artichoke_string_getlen(state, ptr);
  const bytes = [];
  for (let idx = 0; idx < len; idx++) {
    let byte = window._artichoke_string_getch(state, ptr, idx);
    bytes.push(byte);
  }
  return new TextDecoder().decode(new Uint8Array(bytes));
};

const write_string = (state, s) => {
  const ptr = window._artichoke_string_new(state);
  const bytes = new TextEncoder().encode(s);
  for (let idx = 0; idx < bytes.length; idx++) {
    let byte = bytes[idx];
    window._artichoke_string_putch(state, ptr, byte);
  }
  return ptr;
};

const eval_ruby = source => {
  const code = write_string(artichoke, source);
  const output = window._artichoke_eval(artichoke, code);
  const result = read_string(artichoke, output);
  window._artichoke_string_free(artichoke, code);
  window._artichoke_string_free(artichoke, output);
  return result;
};

const playground_run = () => {
  const editor = ace.edit("editor");
  const source = editor.getValue();
  const output = eval_ruby(source);
  document.getElementById("output").innerText = output;
};

window._artichoke_build_info = () => read_string(window.artichoke, 0);
window._artichoke_playground_eval = playground_run;
