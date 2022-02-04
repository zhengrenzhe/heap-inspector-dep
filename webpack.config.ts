import { resolve } from "path";
import { Configuration as WebpackCfg } from "webpack";
import { Configuration as DevServeCfg } from "webpack-dev-server";
import HtmlWebpackPlugin from "html-webpack-plugin";
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";

const env = process.env.NODE_ENV as "production" | "development";

console.log(`use ${env} mode`);

const base: WebpackCfg & DevServeCfg = {
  mode: env,
  devtool: env === "development" ? "eval-source-map" : false,
  output: {
    path: resolve(__dirname, "./dist"),
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
    extensions: [".tsx", ".ts", ".less", ".js", ".jsx"],
    alias: {
      "@": resolve(__dirname, "./web"),
      "@wasm": resolve(__dirname, "./wasm/pkg"),
    },
  },
};

const main: WebpackCfg & DevServeCfg = Object.assign({}, base, {
  entry: {
    web: resolve(__dirname, "./web/web.tsx"),
    background: resolve(__dirname, "./web/background.ts"),
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
      template: resolve(__dirname, "./web/web.html"),
    }),
    new WasmPackPlugin({
      crateDirectory: resolve(__dirname, "./wasm"),
      watchDirectories: [
        resolve(__dirname, "./wasm/src"),
        resolve(__dirname, "./wasm/snapshot_parser/src"),
      ],
      extraArgs: "--target web --mode normal",
      forceMode: "production",
    }),
  ],
});

const worker: WebpackCfg & DevServeCfg = Object.assign({}, base, {
  entry: {
    worker: resolve(__dirname, "./web/worker.ts"),
  },
  target: "webworker",
});

export default [main, worker];
