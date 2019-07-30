import "bootstrap";
import "bootstrap/dist/css/bootstrap.min.css";
import ace from "ace-builds";
import "ace-builds/webpack-resolver";
import "artichoke-wasm/artichoke_wasm.wasm";
import "artichoke-wasm/deps/artichoke-wasm.js";

const sample = `
# https://ruby-doc.org/stdlib-2.5.1/libdoc/forwardable/rdoc/Forwardable.html
require 'forwardable'
# https://ruby-doc.org/stdlib-2.6.3/libdoc/set/rdoc/Set.html
require 'set'

class Registry
  extend Forwardable
  def_delegators :@records, :add, :to_a

  def initialize
    @records = Set.new
  end
end

registry = Registry.new

3.times do
  10.times do |record|
    registry.add(record)
  end
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
