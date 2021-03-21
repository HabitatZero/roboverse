<script lang="ts">
  import Shapes from './InsertOptions/Shapes.svelte';
  import Lights from './InsertOptions/Lights.svelte';
  import InsertEtc from './InsertOptions/Etc.svelte';

  import Box from './SimpleShapesOptions/Box.svelte';
  import Sphere from './SimpleShapesOptions/Sphere.svelte';
  import Cylinder from './SimpleShapesOptions/Cylinder.svelte';

  import Point from './LightsOptions/Point.svelte';
  import Spot from './LightsOptions/Spot.svelte';
  import Directional from './LightsOptions/Directional.svelte';

  import Models from './ModelsOptions/Model.svelte';
</script>

<div class="panelsGroup">
  <div id="insertMenu" class="leftPanels insertMenus ui-overlay-shadow">
    <div class="clickable panelTitle closePanels" title="Insert">
      <h3>Insert</h3>
    </div>
    <div class="panelContent">
      <Shapes />
      <Lights />
      <InsertEtc />
    </div>
  </div>
  <div id="insertMenu-simple_shapes" class="leftPanels insertMenus insertCategory ui-overlay-shadow">
    <div class="clickable panelTitle" title="Simple Shapes">
      <h3>Simple Shapes</h3>
      <img src="style/images/back.png" />
    </div>
    <div class="panelContent">
      <Box />
      <Sphere />
      <Cylinder />
    </div>
  </div>
  <div id="insertMenu-lights" class="leftPanels insertMenus insertCategory ui-overlay-shadow">
    <div class="clickable panelTitle" title="Lights">
      <h3>Lights</h3>
      <img src="style/images/back.png" />
    </div>
    <div class="panelContent">
      <Point />
      <Spot />
      <Directional />
    </div>
  </div>
  <div ng-repeat="category in categories" id="insertMenu-category-placeholder"
    class="leftPanels insertMenus insertCategory ui-overlay-shadow">
    <div class="clickable panelTitle" title="category-placeholder-title">
      <h3>category-placeholder</h3>
      <img src="style/images/back.png" />
    </div>
    <div class="panelContent">
      <Models />
    </div>
  </div>
</div>
