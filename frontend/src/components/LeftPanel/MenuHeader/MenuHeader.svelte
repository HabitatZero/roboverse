<script lang="ts">
  import { ChevronDownIcon } from 'svelte-feather-icons';
</script>

<style lang="scss">
  @import "variables";

  h3 {
    display: block;
    width: calc(100% - 8px);
    height: 100%;
    border: 1px solid black;
    border-left: 0;
    border-right: 0;
    padding: $space/2 $space/4;
    margin: 0;

    background-color: $primary-3;
    color: $primary-5;
    font-size: $baseline;
  }
</style>

<h3 class="menu-header">
  <ChevronDownIcon size="16" />
  <slot></slot>
</h3>
