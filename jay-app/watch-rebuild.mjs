#!/usr/bin/env node
import chokidar from "chokidar";
import { exec } from "child_process";
import { promisify } from "util";

const execAsync = promisify(exec);

async function rebuild() {
    console.log("🐦 Rebuilding with Jay...");
    try {
        await execAsync("npx jay build", { stdio: "inherit" });
        console.log("✅ Rebuild complete.");
    } catch (err) {
        console.error("❌ Rebuild failed:", err);
    }
}

rebuild();

chokidar
    .watch(["src/components", "src/pages"], {
        ignoreInitial: true,
    })
    .on("add", rebuild)
    .on("change", rebuild)
    .on("unlink", rebuild);