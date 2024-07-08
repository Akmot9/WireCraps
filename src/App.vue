<template>
  <div>
    <header>
      <h1>WireCraps</h1>
    </header>
  
    <main>
      <section>
        <h2>Open Recent Capture</h2>
        <ul>
          <li v-for="(file, index) in folder" :key="index">{{ file }}</li>
        </ul>
      </section>
      <section>
        <h2>Capture Interfaces</h2>
        <ul>
          <button v-for="(iface, index) in interfaces" :key="index">{{ iface }}</button>
        </ul>
      </section>
    </main>

    <nav>
      <h2>About WireCraps</h2>
      <ul>
        <li><a href="#">User Guide</a></li>
        <li><a href="#">Wiki</a></li>
        <li><a href="#">Discover</a></li>
      </ul>
    </nav>
  </div>
</template>

<style scoped>
body {
  font-family: Arial, sans-serif;
  margin: 0;
  padding: 0;
  background-color: #f0f0f0;
}

header {
  background-color: #0070c0;
  padding: 15px;
  text-align: center;
  color: white;
  border-bottom: 1px solid #004a80;
}

header h1 {
  margin: 0;
  font-size: 24px;
}

nav {
  margin: 20px 0;
  padding: 10px;
  background-color: #e6e6e6;
  border: 1px solid #ccc;
  border-radius: 5px;
}

nav h2 {
  color: #0070c0;
}

nav ul {
  list-style-type: none;
  padding: 0;
}

nav ul li {
  padding: 10px;
}

nav ul li a {
  color: #0070c0;
  text-decoration: none;
}

nav ul li a:hover {
  text-decoration: underline;
}

main {
  padding: 20px;
  background-color: #fff;
  border: 1px solid #ccc;
  border-radius: 5px;
}

h2 {
  color: #0070c0;
}

ul {
  list-style-type: none;
  padding: 0;
}

li {
  padding: 5px;
  border-bottom: 1px solid #ddd;
}

button {
  display: block;
  width: 100%;
  padding: 10px;
  margin: 5px 0;
  background-color: #0070c0;
  color: white;
  border: none;
  border-radius: 3px;
  cursor: pointer;
}

button:hover {
  background-color: #005fa3;
}
</style>


<script>
import { invoke } from "@tauri-apps/api/core";
import { trace, info, error, attachConsole } from "@tauri-apps/plugin-log";

export default {
  data() {
    return {
      interfaces: [],
      folder: ['1.pcap','2.pcap']
    };
  },
  async created() {
    try {
      this.interfaces = await invoke('get_interfaces');
    } catch (error) {
      console.error('Failed to fetch network interfaces:', error);
    }
  }
};
</script>

<style scoped>
h1 {
  color: #42b983;
}
</style>
