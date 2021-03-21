<script lang="ts">
  import Edit from './Options/Edit.svelte';
  import View from './Options/View.svelte';
  import Options from './Options/Options.svelte';
</script>

<div id="mainMenu" class="leftPanels ui-overlay-shadow">
  <div class="clickable panelTitle closePanels">
    <h3>Menu</h3>
  </div>
  <div class="panelContent">
    <ul>
      <Edit />
      <View />
      <Options />
    </ul>
  </div>
</div>
