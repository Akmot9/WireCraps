<script>
import Greet from "./components/Greet.vue";
import { trace, info, error, attachConsole } from "@tauri-apps/plugin-log";
import { invoke } from "@tauri-apps/api/core";

export default {
  data() {
    return {
      name: '',
      message: 'Hello World!'
    };
  },
  components: {
    Greet
  },
  async mounted() {
    this.fetchHostname();
  },
  async created() {
    this.initializeLogging();
  },
  methods: {
    async fetchHostname() {
      try {
        this.name = await invoke("get_hostname");
      } catch (err) {
        error(`Failed to get hostname: ${err}`);
      }
    },
    async initializeLogging() {
      try {
        const detach = await attachConsole();
        info("Info");
        detach();
      } catch (err) {
        error(`Failed to attach console: ${err}`);
      }
    }
  }
}
</script>

<template>
  <div class="container">
    <h1>Welcome to Tauri! {{ name }}</h1>

    <div class="row">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>

    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <p>
      Recommended IDE setup:
      <a href="https://code.visualstudio.com/" target="_blank">VS Code</a>
      +
      <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
      +
      <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">Tauri</a>
      +
      <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">rust-analyzer</a>
    </p>

    <Greet />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
