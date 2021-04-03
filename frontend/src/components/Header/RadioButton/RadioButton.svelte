<style lang="scss">
  @import "variables";

  .radio-button {
    display: inline-block;
    background-color: $primary-4;

    border-style: solid;
    border-color: $primary-3;
    border-radius: $space/4;
    border-width: 1px;
    border-radius: 0;

    color: white;
    cursor: pointer;
    width: $space*2.5;
    height: $space*2.5;

    margin-right: -4px;

    &:active {
      background-color: $primary-5;
      border: 1px solid $primary-4;
    }

    &:first-of-type {
      border-width: 1px 0 1px 1px;
      border-radius: $space/4 0 0 $space/4;
    }

    &:last-of-type {
      border-width: 1px 1px 1px 0;
      border-radius: 0 $space/4 $space/4 0;
    }
  }
</style>

<div class="radio-button">
  <slot></slot>
</div>
