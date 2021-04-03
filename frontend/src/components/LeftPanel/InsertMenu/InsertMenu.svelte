<script lang="ts">
  import InsertOptions from './InsertOptions/InsertOptions.svelte';
  import SimpleShapesOptions from './SimpleShapesOptions/SimpleShapesOptions.svelte';
  import LightOptions from './LightsOptions/LightsOptions.svelte';
  // import ModelsOptions from './ModelsOptions/ModelsOptions.svelte';
</script>

<style lang="scss">
  #left-panel {
    display: none;
  }
</style>

<div id="left-panel" class="panels-group">
  <InsertOptions />
  <SimpleShapesOptions />
  <LightOptions />
  <!-- <ModelsOptions /> -->
</div>
