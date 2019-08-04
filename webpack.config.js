const path = require("path");
const HtmlWebPackPlugin = require("html-webpack-plugin");
const HtmlWebpackInlineSourcePlugin = require("html-webpack-inline-source-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const OptimizeCSSAssetsPlugin = require("optimize-css-assets-webpack-plugin");

const plugins = [
  new MiniCssExtractPlugin({
    filename: "[name].css",
    chunkFilename: "[id].css"
  }),
  new HtmlWebPackPlugin({
    template: "index.html",
    filename: "index.html",
    minify: {
      collapseWhitespace: true,
      minifyCSS: true,
      minifyJS: true,
      removeComments: true,
      useShortDoctype: true
    }
  }),
  new HtmlWebPackPlugin({
    template: "regexp-benchmark.html",
    filename: "benchmarks/regexp/index.html",
    minify: {
      collapseWhitespace: true,
      minifyCSS: true,
      minifyJS: true,
      removeComments: true,
      useShortDoctype: true
    }
  }),
  new HtmlWebpackInlineSourcePlugin()
];

module.exports = (env, argv) => {
  let target = "debug";
  let cssLoader = "style-loader";
  if (argv.mode === "production") {
    target = "release";
    cssLoader = MiniCssExtractPlugin.loader;
  }
  return {
    context: path.resolve(__dirname, "artichoke-wasm/src"),
    resolve: {
      alias: {
        "artichoke-bench": path.resolve(__dirname, `target/bench`),
        "artichoke-wasm": path.resolve(
          __dirname,
          `target/wasm32-unknown-emscripten/${target}`
        )
      }
    },
    entry: path.resolve(__dirname, "artichoke-wasm/src/playground.js"),
    output: {
      path: path.resolve(__dirname, `target/webpack/${target}`),
      publicPath: "/artichoke/"
    },
    module: {
      rules: [
        {
          test: /\.jsx?$/,
          exclude: /node_modules/,
          use: {
            loader: "babel-loader"
          }
        },
        {
          test: /\.css$/,
          use: [cssLoader, "css-loader"]
        },
        {
          test: /\.(jpe?g|png|gif)$/,
          use: ["url-loader", "image-webpack-loader"]
        },
        {
          test: /@artichoke\/logo\/logo\.svg/,
          use: [
            {
              loader: "file-loader",
              options: {
                name: "[name].[ext]"
              }
            },
            {
              loader: "svgo-loader"
            }
          ]
        },
        {
          test: /\.svg$/,
          exclude: /@artichoke\/logo\/logo\.svg/,
          use: ["svg-url-loader", "svgo-loader"]
        },
        {
          test: /\.(rb|txt)$/,
          use: ["raw-loader"]
        },
        {
          test: /-wasm\.js$/,
          use: ["uglify-loader", "script-loader"]
        },
        {
          test: /\.wasm$/,
          type: "javascript/auto",
          use: [
            {
              loader: "file-loader",
              options: {
                name: "[name].[ext]"
              }
            }
          ]
        }
      ]
    },
    plugins,
    optimization: {
      minimizer: [new TerserPlugin(), new OptimizeCSSAssetsPlugin()]
    }
  };
};
