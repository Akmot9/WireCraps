<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { trace, info, error, attachConsole } from "@tauri-apps/plugin-log";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  info(name.value)
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("hello", { name: name.value });
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>
</template>
