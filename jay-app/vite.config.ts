import { defineConfig } from "vite";
import path from "node:path";

export default defineConfig({
  root: "src", // Where your `index.html` lives
  build: {
    outDir: "../dist",
    emptyOutDir: true,
    rollupOptions: {
      input: path.resolve(__dirname, "src/index.html"),
    },
  },
  resolve: {
    alias: {
      "@components": path.resolve(__dirname, "src/components", "src/index.html"),
    },
  },
});