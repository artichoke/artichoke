import "bootstrap";
import "bootstrap/dist/css/bootstrap.min.css";
import ace from "ace-builds";
import "ace-builds/webpack-resolver";
import "artichoke-wasm/artichoke_wasm.wasm";
import "artichoke-wasm/deps/artichoke-wasm.js";

ace.edit("editor", {
  mode: "ace/mode/ruby",
  theme: "ace/theme/monokai",
  fontSize: 14
});
