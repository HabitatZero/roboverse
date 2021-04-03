<script lang="ts">
  import Edit from './Options/Edit.svelte';
  import View from './Options/View.svelte';
  import Options from './Options/Options.svelte';
</script>

<style lang="scss">
  @import "variables";

  .main-menu {
    display: block;
  }

  h2 {
    padding: $space/2;
    margin: 0;
    font-size: $baseline*1.25;
  }
</style>

<div class="main-menu">
  <h2>Menu</h2>
  <Edit />
  <View />
  <Options />
</div>
