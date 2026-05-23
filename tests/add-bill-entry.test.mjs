import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const appVue = readFileSync(resolve("src/App.vue"), "utf8");
const plusButton = appVue.match(/<v-btn[\s\S]*?icon="mdi-plus"[\s\S]*?>/)?.[0] ?? "";

assert.ok(
  plusButton.includes('to="/assess"') || plusButton.includes(":to=\"'/assess'\""),
  "The top-right add button should navigate to the /assess add-bill route.",
);

assert.doesNotMatch(
  appVue,
  /<v-dialog[\s\S]*?<AddBill[\s\S]*?<\/v-dialog>/,
  "App.vue should not keep a second AddBill dialog entry point.",
);
