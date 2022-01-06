import { resolve } from "path";
import { Configuration as WebpackCfg } from "webpack";
import { Configuration as DevServeCfg } from "webpack-dev-server";
import HtmlWebpackPlugin from "html-webpack-plugin";
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";

const cfg: WebpackCfg & DevServeCfg = {
  mode: "development",
  devtool: "source-map",
  entry: {
    web: resolve(__dirname, "web/web.ts"),
    background: resolve(__dirname, "web/background.ts"),
  },
  output: {
    path: resolve(__dirname, "dist"),
    filename: "[name].js",
  },
  module: {
    rules: [
      {
        test: /\.(less|css)$/i,
        use: ["style-loader", "css-loader", "less-loader"],
      },
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".ts", ".tsx", ".less", ".js", ".jsx"],
  },
  devServer: {
    port: 3000,
    host: "0.0.0.0",
  },
  experiments: {
    asyncWebAssembly: true,
  },
  plugins: [
    new HtmlWebpackPlugin({
      chunks: ["web"],
      filename: "index.html",
      inject: "body",
      template: resolve(__dirname, "web/web.html"),
    }),
    new WasmPackPlugin({
      crateDirectory: resolve(__dirname),
      watchDirectories: [resolve(__dirname, "src")],
      extraArgs: "--target web --mode normal",
    }),
  ],
};

export default cfg;
