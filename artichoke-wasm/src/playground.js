import "bootstrap";
import "bootstrap/dist/css/bootstrap.min.css";
import ace from "ace-builds";
import "ace-builds/webpack-resolver";
import "artichoke-wasm/artichoke_wasm.wasm";
import "artichoke-wasm/deps/artichoke-wasm.js";

const sample = `
class A
  def bar(x)
    x.to_s
  end
end

10.times do |i|
    puts A.new.bar(i)
end
`;

ace.edit("editor", {
  mode: "ace/mode/ruby",
  theme: "ace/theme/monokai",
  fontSize: 14
});

ace.edit("editor").setValue(sample.trim(), -1);
