<style lang="scss">
  @import "variables";

  .menu-option {
    display: block;
    border-bottom: 1px solid black;
    padding: $space/4 $space/2;
    width: calc(100%  - 32px);

    background-color: $primary-4;
    padding-left: $space + 8px;

    &:last-of-type {
      border-bottom: 0;
    }

    font-size: $baseline/1.25;
  }
</style>

<span class="menu-option">
  <slot></slot>
</span>
