<template>
  <div>
    <header>
      <h1>Welcome to WireCraps</h1>
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
          <li v-for="(iface, index) in interfaces" :key="index">{{ iface }}</li>
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
header {
  background-color: #42b983;
  padding: 10px;
  text-align: center;
  color: white;
}

nav {
  margin: 20px 0;
  padding: 10px;
  background-color: #333;
}

nav h2 {
  color: white;
}

nav ul {
  list-style-type: none;
  padding: 0;
}

nav ul li {
  padding: 10px;
}

nav ul li a {
  color: white;
  text-decoration: none;
}

nav ul li a:hover {
  text-decoration: underline;
}

main {
  padding: 20px;
}

h1 {
  color: white;
}

h2 {
  color: #42b983;
}

ul {
  list-style-type: none;
  padding: 0;
}

li {
  padding: 5px;
  border-bottom: 1px solid #ddd;
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
