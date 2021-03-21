<script lang="ts">
  import LeftPanel from '../LeftPanel/LeftPanel.svelte'
  // import Tabs from '../Tabs/Tabs.svelte'
  // import Header from '../Header/Header.svelte'
  // import LogPlay from '../LogPlay/LogPlay.svelte'
  // import Content from '../Content/Content.svelte'
</script>

<style lang="scss">
  @import "variables";
  @import url('https://fonts.googleapis.com/css2?family=Source+Sans+Pro&display=swap');

  :global(body) {
    background-color: $base-color;
    color:white;
    font-family: 'Source Sans Pro', sans-serif;
  }
</style>

<main class="custom-component">
  <LeftPanel />
  <!-- <Tabs /> -->
  <!-- <Header /> -->
  <!-- <LogPlay /> -->
  <!-- <Content /> -->
</main>
