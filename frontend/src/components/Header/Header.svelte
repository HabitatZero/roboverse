<script lang="ts">
  import PlayButton from "./PlayButton/PlayButton.svelte";
  import ClockButton from "./ClockButton/ClockButton.svelte";
  import Clock from "./Clock/Clock.svelte";
  import ManipulatorModes from "./ManipulatorModes/ManipulatorModes.svelte";

  import InsertBox from "./InsertBox/InsertBox.svelte";
  import InsertSphere from "./InsertSphere/InsertSphere.svelte";
  import InsertCylinder from "./InsertCylinder/InsertCylinder.svelte";

  import InsertPointLight from "./InsertPointLight/InsertPointLight.svelte";
  import InsertSpotLight from "./InsertSpotLight/InsertSpotLight.svelte";
  import InsertDirectionalLight from "./InsertDirectionalLight/InsertDirectionalLight.svelte";
</script>

<style lang="scss">
  @import "variables";

  .header {
    position: absolute;
    right: 0;
    display: flex;

    margin: $space/2;
  }
</style>

<div class="header">
  <PlayButton></PlayButton>
  <ClockButton></ClockButton>
  <Clock></Clock>

  <ManipulatorModes />

  <InsertBox />
  <InsertSphere />
  <InsertCylinder />

  <InsertPointLight />
  <InsertSpotLight />
  <InsertDirectionalLight />
</div>
