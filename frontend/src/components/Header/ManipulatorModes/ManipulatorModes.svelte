<script>
  import RadioButtonSet from "../RadioButtonSet/RadioButtonSet.svelte";
  import RotationMode from "./RotationMode.svelte";
  import TranslationMode from "./TranslationMode.svelte";

  import ViewMode from "./ViewMode.svelte";
</script>

<RadioButtonSet>
  <ViewMode />
  <TranslationMode />
  <RotationMode />
</RadioButtonSet>
