<script lang="ts">
  import MainMenu from './MainMenu/MainMenu.svelte';
  import InsertMenu from './InsertMenu/InsertMenu.svelte';
  // import SceneTree from './SceneTree/SceneTree.svelte';
</script>

<style lang="scss">
  #left-panel {
    display: block;
  }
</style>

<div id="left-panel">
  <MainMenu />
  <InsertMenu />
  <!-- <SceneTree /> -->
</div>
