<script lang="ts">
  import Shapes from './Shapes/Shapes.svelte';
  import Lights from './Lights/Lights.svelte';
  // import InsertEtc from './Etc/Etc.svelte';
</script>

<div id="insertMenu" class="leftPanels insertMenus ui-overlay-shadow">
  <div class="clickable panelTitle closePanels" title="Insert">
    <h3>Insert</h3>
  </div>
  <div class="panelContent">
    <Shapes />
    <Lights />
    <!-- <InsertEtc /> -->
  </div>
</div>
