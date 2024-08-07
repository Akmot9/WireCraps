<!-- YourVueComponent.vue -->
<script >
  import { VNetworkGraph, VEdgeLabel } from "v-network-graph"
  import { invoke } from '@tauri-apps/api/core';
  import { save } from '@tauri-apps/plugin-dialog';
  import * as vNG from "v-network-graph"
  import {ForceLayout} from "v-network-graph/lib/force-layout"

  export default {
  components: {
    VNetworkGraph,
    VEdgeLabel
    
  },
  data() {
  return {
    position: { left: "0", top: "0" },
    graphData: {
      nodes: [],
      edges: [],
    },
    selectedNode: null,
    viewMenu: null, // Utilisez les refs pour les éléments de menu
    nodeMenu: null,
    edgeMenu: null,
    menuTargetNode: [], // Pour stocker le nœud ciblé par le menu contextuel
    menuTargetEdges: [], // Pour stocker les arêtes ciblées par le menu contextuel
    packets: [],
    intervalId: null,
    configs: vNG.defineConfigs({
      view: {
        maxZoomLevel: 5,
        minZoomLevel: 0.1,
        
        layoutHandler: new ForceLayout({}),
      },
      node: {
        selectable: true,
        normal: { 
          radius: 20,
          color: node => node.color
         }, // Light grey for visibility on dark background
        label: { 
          visible: true,
          color: "#E0E0E0",
          fontSize: 18,
          directionAutoAdjustment: true,
        }, // Same as node color for consistency
      },
      edge: {
        gap: 50,
        type: "curve",
        selectable: true,
        hoverable: true,
        normal: {
          width: 2, // Ou toute autre largeur par défaut que vous souhaitez
          color: edge => { // Ici, vous définissez dynamiquement la couleur de l'arête
            switch(edge.label) {
              case 'Arp':
                return 'yellow';
              case 'Ipv4':
                return 'orange';
              // Ajoutez d'autres cas selon vos besoins
              default:
                return 'white'; // Couleur par défaut
            }
          },
        },
        label: { // Configuration du label conservée telle quelle
          fontFamily: undefined,
          fontSize: 21,
          lineHeight: 1.1,
          color: "#E0E0E0",
          margin: 4,
          background: {
            visible: true,
            color: "#000000",
            padding: {
              vertical: 1,
              horizontal: 4,
            },
            borderRadius: 2,
          },
        },
      },
    }),
  };
},

  computed: {
    processedPackets() {
      return this.processData(this.packets);
    },
  
  },
  mounted() {
    this.intervalId = setInterval(this.fetchPacketInfos, 1000);
    this.viewMenu = this.$refs.viewMenu;
    this.nodeMenu = this.$refs.nodeMenu;
    this.edgeMenu = this.$refs.edgeMenu;

    this.installationName = this.$route.params.installationName;
    this.niveauConfidentialite = this.$route.params.confidentialite;
  },
  beforeDestroy() {
    if (this.intervalId) {
      clearInterval(this.intervalId);
    }
  },
  methods: {
    async fetchPacketInfos() {
      try {
        const jsonString = await invoke('get_graph_state', {});
        this.graphData = JSON.parse(jsonString);

      } catch (error) {
        console.error("Error fetching packet infos:", error);
      }
    },
    getCurrentDate() {
      // Fonction pour obtenir la date actuelle
      const now = new Date();
      // Formattez la date en DD/MM/YYYY
      const formattedDate = `${now.getFullYear()}${this.padZero(now.getMonth() + 1)}${this.padZero(now.getDate())}`;
      return formattedDate;
    },
    padZero(value) {
      // Fonction pour ajouter un zéro en cas de chiffre unique (par exemple, 5 -> 05)
      return value < 10 ? `0${value}` : value;
    },
    async downloadSvg() {
      if (this.$refs.graphnodes && this.$refs.graphnodes.exportAsSvgText) {
        try {
          console.log('Attempting to export SVG...');
          const svgContent = await this.$refs.graphnodes.exportAsSvgText();
          console.log('SVG content:', svgContent);
          
          const url = URL.createObjectURL(new Blob([text], { type: "octet/stream" }))
          console.log('Blob URL created:', url);

          const a = document.createElement("a")
          a.href = url
          a.download = `${this.getCurrentDate()}_${this.niveauConfidentialite}_${this.installationName}.svg`;
          console.log('Download link created:', a);
          a.click()
          window.URL.revokeObjectURL(url)
          console.log('Download initiated and URL revoked.');
          // Use Tauri's dialog API to open a save file dialog
          save({
            filters: [{
              name: 'SVG File',
              extensions: ['svg']
            }],
            defaultPath: this.getCurrentDate()+ '_' + this.niveauConfidentialite  + '_' + this.installationName+ '.svg' // Set the default file name here
          }).then((filePath) => {
            if (filePath) {
              // Use Tauri's fs API to write the file
              invoke('write_file', { path: filePath, contents: svgContent })
                .then(() => console.log('SVG successfully saved'))
                .catch((error) => console.error('Error saving SVG:', error));
            }
          });
        } catch (error) {
          console.error('Error exporting SVG:', error);
        }
      } else {
        console.error('SVG export function not available or graph component not loaded.');
      }
    },
    async downloadSvg() {
      if (this.$refs.graphnodes && this.$refs.graphnodes.exportAsSvgText) {
        try {
          console.log('Attempting to export SVG...');
          const svgContent = await this.$refs.graphnodes.exportAsSvgText();
          save({
            filters: [{
              name: 'SVG File',
              extensions: ['svg']
            }],
            defaultPath: this.getCurrentDate()+ '_' + this.niveauConfidentialite  + '_' + this.installationName+ '.svg' // Set the default file name here
          }).then((filePath) => {
            if (filePath) {
              // Use Tauri's fs API to write the file
              invoke('write_file', { path: filePath, contents: svgContent })
                .then(() => console.log('SVG successfully saved'))
                .catch((error) => console.error('Error saving SVG:', error));
            }
          });
        } catch (error) {
          console.error('Error exporting SVG:', error);
        }
      } else {
        console.error('SVG export function not available or graph component not loaded.');
      }
    },
    async downloadPng() {
      if (this.$refs.graphnodes && this.$refs.graphnodes.exportAsSvgText) {
        try {
          const svgContent = await this.$refs.graphnodes.exportAsSvgText();
          
          // Use Tauri's dialog API to open a save file dialog
          save({
            filters: [{
              name: 'PNG File',
              extensions: ['png']
            }],
            defaultPath: this.getCurrentDate()+ '_' + this.niveauConfidentialite  + '_' + this.installationName+ '.png' // Set the default file name here
          }).then((filePath) => {
            if (filePath) {
              // Use Tauri's fs API to write the file
              invoke('write_file', { path: filePath, contents: svgContent })
                .then(() => console.log('png successfully saved'))
                .catch((error) => console.error('Error saving png:', error));
            }
          });
        } catch (error) {
          console.error('Error exporting png:', error);
        }
      } else {
        console.error('png export function not available or graph component not loaded.');
      }
      },
    handleNodeClick(node) {
      // Supposons que `selectedNode` est une propriété de données que vous utiliserez pour stocker les informations du node sélectionné
      this.selectedNode = node;
      console.log('selected node:',this.selectedNode)
    },
    showContextMenu(element, event) {
      console.log('element', element)
      console.log('event', event)

      element.style.left = event.x + "px"
      element.style.top = event.y + "px"
      element.style.visibility = "visible"
      const handler = (event) => {
        if (!event.target || !element.contains(event.target)) {
          element.style.visibility = "hidden"
          document.removeEventListener("pointerdown", handler, { capture: true })
        }
      }
      document.addEventListener("pointerdown", handler, { passive: true, capture: true })
    },

    showEdgeContextMenu({ edge, event }) {
      if (this.edgeMenu) {
        const edgeData = this.graphData.edges[edge];
        this.menuTargetEdges = [
          `Adresse Mac Source: ${edgeData.source}, 
          Adresse Mac Destination: ${edgeData.target}, 
          Protocole: ${edgeData.label}`
        ];
        this.showContextMenu(this.edgeMenu, event);
      }
    },
    showNodeContextMenu({ node, event }) {
      if (this.nodeMenu) {
        const nodeData = this.graphData.nodes[node];
        this.menuTargetNode = [
          `Adresse IP: ${nodeData.name},
          Mac_address: ${nodeData.mac}`
        ];
        this.showContextMenu(this.nodeMenu, event);
      }
    },
  }
}
</script>

<template>
  <div class="graph-container">
    
    <button class="download-button" @click="downloadPng">Télécharger l'image</button>
    <button class="download-button" @click="downloadSvg" style="left: 200px;">Télécharger SVG</button>
    
    <v-network-graph
      class="graph"
      ref="graphnodes"
      zoom-level=3
      :nodes="graphData.nodes"
      :edges="graphData.edges"
      :layouts="graphData.layouts"
      :configs="configs"
      :event-handlers="{
        'node:click': showNodeContextMenu,
        'edge:click': showEdgeContextMenu,
      }"
    >
      <template #edge-label="{ edge, ...slotProps }">
        <v-edge-label :text="edge.label" align="above" v-bind="slotProps" />
      </template>
    </v-network-graph>
  </div>
  <div ref="nodeMenu" class="context-menu">
    Infos du noeud:
    <ul class="contenu">
      <li v-for="(info, index) in menuTargetNode" :key="index">{{ info }}</li>
    </ul>
  </div>
  <div ref="edgeMenu" class="context-menu">
    Infos de l'arête:
    <div class="contenue">{{ menuTargetEdges.join(", ") }}</div>
  </div>
</template>

<style scoped>
.graph-container {
  position: relative; /* Establishes a relative positioning context */
  height: 850px; /* Adjust height as needed */
  width: 100%; /* Container takes full width */
}

.graph {
  height: 100%; /* Graph takes full height of the container */
  border: 2px solid #3a3a3a;
  border-radius: 10px;
  width: 100%;
  text-align: center;
  color: #FFF;
  box-shadow: 0 10px 20px rgba(0, 0, 0, 0.5);
  background-color: #1a1a1a;
}

.download-button {
  position: absolute; /* Absolutely positioned relative to its nearest positioned ancestor */
  top: 10px; /* Distance from the top of the container */
  left: 10px; /* Distance from the left of the container */
  background-color: #0b1b25;
  color: #fff;
  padding: 10px 20px;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  z-index: 10; /* Ensure the button is above the graph */
}

.context-menu {
  color: #0b1b25;
  border-radius: 10px;
  width: 180px;
  background-color: #efefef;
  padding: 10px;
  position: fixed;
  visibility: hidden;
  font-size: 12px;
  border: 1px solid #aaaaaa;
  box-shadow: 2px 2px 2px #e7bf0c;
}

.contenue {
  color: #0b1b25;
  border: 1px dashed #aaa;
  padding: 4px;
  margin-top: 8px;
}

</style>
