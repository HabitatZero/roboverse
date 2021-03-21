<script lang="ts">
  import LeftPanel from '../LeftPanel/LeftPanel.svelte'
  import Tabs from '../Tabs/Tabs.svelte'
  import Header from '../Header/Header.svelte'
  import LogPlay from '../LogPlay/LogPlay.svelte'
  import Content from '../Content/Content.svelte'
</script>

<style lang="scss">
  @import "variables";
  @import url('https://fonts.googleapis.com/css2?family=Roboto&display=swap');

  :global(body) {
    background-color: $base-color;
    color: $text-color;
    font-family: 'Roboto', sans-serif;
  }

  #roboverse {
    display: flex;
  }

  .left-menus {
    display: flex;
    min-width: 200px;
    width: 30%;
  }
</style>

<main id="roboverse">
  <div class="left-menus">
    <LeftPanel />
    <Tabs />
  </div>
  <div class="main-content">
    <Header />
    <LogPlay />
    <Content />
  </div>
</main>
