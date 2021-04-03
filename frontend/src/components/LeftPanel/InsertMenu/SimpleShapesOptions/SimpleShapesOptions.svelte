<script type="ts">
  import Box from './Box/Box.svelte';
  import Sphere from './Sphere/Sphere.svelte';
  import Cylinder from './Cylinder/Cylinder.svelte';
</script>

<div id="insertMenu-simple_shapes" class="leftPanels insertMenus insertCategory ui-overlay-shadow">
  <div class="clickable panelTitle" title="Simple Shapes">
    <h3>Simple Shapes</h3>
    <img src="./images/back.png" />
  </div>
  <div class="panelContent">
    <Box />
    <Sphere />
    <Cylinder />
  </div>
</div>
