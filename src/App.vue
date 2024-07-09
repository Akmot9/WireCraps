<template>
  <div class="bg"></div>
  <router-view></router-view>
</template>

<script>
import { getCurrent } from "@tauri-apps/api/window";
import { confirm } from '@tauri-apps/plugin-dialog';

export default {
  data() {
    return {
      // Add a data property for the unlisten function
      unlistenCloseEvent: null,
    };
  },

  async mounted() {
    console.log("mounted");

    // Set up the close event listener
    this.unlistenCloseEvent = await getCurrent().onCloseRequested(async (event) => {
      const confirmed = await confirm(
        'Etes vous sûr ?',
        { title: 'Tauri', kind: 'warning' }
      );
      if (!confirmed) {
        // User did not confirm closing the window; let's prevent it
        event.preventDefault();
      }
    });
  },

  beforeUnmount() {
    // Call the unlisten function when the component is unmounted
    if (this.unlistenCloseEvent) {
      this.unlistenCloseEvent();
    }
  }
};
</script>

<style scoped>
/* Global styles for your app */
</style>
