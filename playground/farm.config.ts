import { defineConfig } from "@farmfe/core";
import react from "@farmfe/plugin-react";
import farmPlugin from "farm-plugin-console-clear";

export default defineConfig({
  compilation: {
    input: {
      index: "./index.html",
    },
    // output: {
    //   targetEnv: "browser-esnext",
    // },
    persistentCache: false,
    progress: false,
    runtime: {
      isolate: true,
    },
  },
  plugins: [react({ runtime: "automatic" }), farmPlugin()],
});
