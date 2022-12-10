import { resolve } from "path";
import HtmlWebpackPlugin from "html-webpack-plugin";
import TerserPlugin from "terser-webpack-plugin";
import { Configuration as WebpackCfg } from "webpack";
import { Configuration as DevServeCfg } from "webpack-dev-server";

import pkg from "./package.json";

const base: WebpackCfg & DevServeCfg = {
  mode: "development",
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
      "@web": resolve(__dirname, "./web"),
    },
  },
};

const main: WebpackCfg & DevServeCfg = Object.assign({}, base, {
  entry: {
    web: resolve(__dirname, "./web/page/index.tsx"),
  },
  devServer: {
    port: 3000,
    host: "0.0.0.0",
    headers: {
      "Cross-Origin-Embedder-Policy": "require-corp",
      "Cross-Origin-Opener-Policy": "same-origin",
    },
  },
  experiments: {
    asyncWebAssembly: true,
    topLevelAwait: true,
  },
  optimization: {
    minimize: true,
    minimizer: [
      new TerserPlugin({
        parallel: true,
        terserOptions: {
          keep_classnames: true,
          keep_fnames: true,
        },
      }),
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      chunks: ["web"],
      filename: "index.html",
      inject: "body",
      title: pkg.displayName,
    }),
  ].concat([]),
});

const worker: WebpackCfg & DevServeCfg = Object.assign({}, base, {
  entry: {
    worker: resolve(__dirname, "./web/worker/index.ts"),
  },
  target: "webworker",
});

export default [main, worker];
