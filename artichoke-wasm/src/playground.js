import "bootstrap";
import "bootstrap/dist/css/bootstrap.min.css";
import ace from "ace-builds";
import "ace-builds/webpack-resolver";
import "artichoke-wasm/artichoke_wasm.wasm";
import "artichoke-wasm/deps/artichoke-wasm";
import sample from "./playground.rb";

ace.edit("editor", {
  mode: "ace/mode/ruby",
  theme: "ace/theme/monokai",
  fontSize: 14,
  tabSize: 2,
  useSoftTabs: true
});

ace.edit("editor").setValue(sample.trim(), -1);

const Heap = {
  read(state, ptr) {
    const len = window._artichoke_string_getlen(state, ptr);
    const bytes = [];
    for (let idx = 0; idx < len; idx += 1) {
      const byte = window._artichoke_string_getch(state, ptr, idx);
      bytes.push(byte);
    }
    return new TextDecoder().decode(new Uint8Array(bytes));
  },
  write(state, s) {
    const ptr = window._artichoke_string_new(state);
    const bytes = new TextEncoder().encode(s);
    for (let idx = 0; idx < bytes.length; idx += 1) {
      const byte = bytes[idx];
      window._artichoke_string_putch(state, ptr, byte);
    }
    return ptr;
  }
};

const evalRuby = source => {
  const { artichoke } = window;
  const code = Heap.write(artichoke, source);
  const output = window._artichoke_eval(artichoke, code);
  const result = Heap.read(artichoke, output);
  window._artichoke_string_free(artichoke, code);
  window._artichoke_string_free(artichoke, output);
  return result;
};

const playgroundRun = () => {
  const editor = ace.edit("editor");
  const source = editor.getValue();
  const output = evalRuby(source);
  document.getElementById("output").innerText = output;
};

window._artichoke_build_info = () => Heap.read(window.artichoke, 0);
window._artichoke_playground_eval = playgroundRun;
