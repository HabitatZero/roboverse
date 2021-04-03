<script lang="ts">
  import MainMenu from './MainMenu/MainMenu.svelte';
  import InsertMenu from './InsertMenu/InsertMenu.svelte';
  // import SceneTree from './SceneTree/SceneTree.svelte';
</script>

<style lang="scss">
  @import "variables";

  #left-panel {
    display: block;
    background-color: $primary-5;
    height: 100vh;
  }
</style>

<div id="left-panel">
  <MainMenu />
  <InsertMenu />
  <!-- <SceneTree /> -->
</div>
