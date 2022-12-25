import { defineConfig } from "vite";
import { resolve } from "path";
import typescript from "@rollup/plugin-typescript";
import ttypescript from "ttypescript";

const isProd = process.env["NODE_ENV"] === "production";
console.log(`is prod: ${isProd}`);

export default defineConfig({
  root: "./web",
  resolve: {
    alias: {
      "@web": resolve(__dirname, "./web"),
    },
  },
  plugins: [
    typescript({
      typescript: ttypescript,
      sourceMap: !isProd,
    }),
  ],
  server: {
    hmr: false,
  },
  build: {
    outDir: resolve(__dirname, "./dist"),
    assetsDir: "",
    emptyOutDir: true,
  },
});
