<script lang="ts">
  export let src: string = '';
  export let alt: string = '';
</script>

<style lang="scss">
  @import "variables";

  button {
    background-color: $primary-4;
    border: 1px solid $primary-3;
    border-radius: $space/4;

    color: white;
    cursor: pointer;
    width: $space*2.5;
    height: $space*2.5;

    margin-right: $space/2;

    img {
      height: $space;
      width: $space;
    }

    &:active {
      background-color: $primary-5;
      border: 1px solid $primary-4;
    }
  }
</style>

<button type="button" {alt}>
  <slot></slot>
  {#if src}
    <img {src} {alt}/>
  {/if}
</button>
