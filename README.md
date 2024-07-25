## farm-plugin-console-clear

A rust plugin for [farm](https://github.com/farm-fe/farm) to clear console statement.

### Install

```bash
pnpm add -D farm-plugin-console-clear
```

### Usage

```ts
import { defineConfig } from "@farmfe/core";
import consoleClear from "farm-plugin-console-clear";

interface Options {
  include: string[]; // default: "src/"
  exclude: string[]; // default: "node_modules/"
}

export default defineConfig({
  plugins: [
    consoleClear({
      include: ["src/"],
      exclude: ["node_modules/"],
    }),
  ],
});
```
