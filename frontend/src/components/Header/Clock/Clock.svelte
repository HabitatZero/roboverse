<style scoped lang="scss">
  @import "variables";

  .clock {
    background-color: $primary-4;
    border: 1px solid $primary-3;
    border-radius: $space/4;
    padding: $space/4;
    height: $space*1.89;
    width: $space*9;
    font-size: $baseline/1.25;
    margin-right: $space/2;
  }

  .time {
    display: flex;
    justify-content: space-between;
  }
</style>

<div class="clock">
  <div class="time">
    <span class="time-label">Real Time: </span>
    <span class="time-value">00:00:00.000</span>
  </div>
  <div class="time">
    <span class="time-label">Sim Time: </span>
    <span class="time-value">00:00:00.000</span>
  </div>
</div>
