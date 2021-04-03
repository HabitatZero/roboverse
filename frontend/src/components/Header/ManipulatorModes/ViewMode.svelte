<script lang="ts">
  import RadioButton from "../RadioButton/RadioButton.svelte";
  import { MousePointerIcon } from "svelte-feather-icons";
</script>

<RadioButton>
  <input type="radio" name="radio-mini" id="view-mode" value="choice-1" checked="checked" />
  <MousePointerIcon size="16" alt="View Mode (ESC)" title="View Mode (ESC)" />
</RadioButton>
