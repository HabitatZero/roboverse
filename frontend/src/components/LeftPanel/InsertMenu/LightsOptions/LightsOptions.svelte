<script lang="ts">
  import Point from './Point/Point.svelte';
  import Spot from './Spot/Spot.svelte';
  import Directional from './Directional/Directional.svelte';
</script>

<div id="insertMenu-lights" class="leftPanels insertMenus insertCategory ui-overlay-shadow">
  <div class="clickable panelTitle" title="Lights">
    <h3>Lights</h3>
    <img src="./images/back.png" />
  </div>
  <div class="panelContent">
    <Point />
    <Spot />
    <Directional />
  </div>
</div>
