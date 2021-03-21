<div id="logplay" class="gzGUI" style="display: none">
  <div id="logplay-slider" class="ui-field-contain gzGUI">
    <label for="logplay-slider-input"></label>
    <input type="range" name="logPlay" id="logplay-slider-input" value="0" min="0" max="100" data-mini="true"
      data-highlight="true" class="ui-hidden-accessible" />
  </div>
  <fieldset id="logplay-clock-fieldset" class="gzGUI">
    <p><span class="sim-time-value">00:00:00.000</span></p>
    <p><span class="end-time-value">00:00:00.000</span></p>
  </fieldset>

  <fieldset class="gzGUI logplay-fieldset" id="logplay-rewind-fieldset">
    <a data-role="button" class="header-button" id="logplay-rewind"><span><img src="./images/log_rewind.png"
          title="Rewind" /></span></a>
  </fieldset>
  <fieldset class="gzGUI logplay-fieldset" id="logplay-stepback-fieldset">
    <a data-role="button" class="header-button" id="logplay-stepback"><span><img src="./images/log_step_back.png"
          title="Step back" /></span></a>
  </fieldset>
  <fieldset class="gzGUI logplay-fieldset" id="logplay-play-fieldset">
    <a data-role="button" class="header-button" id="logplay-play"><span id="logplay-playText"><img
          src="./images/play.png" title="Play" /></span></a>
  </fieldset>
  <fieldset class="gzGUI logplay-fieldset" id="logplay-stepforward-fieldset">
    <a data-role="button" class="header-button" id="logplay-stepforward"><span><img
          src="./images/log_step_forward.png" title="Step forward" /></span></a>
  </fieldset>
  <fieldset class="gzGUI logplay-fieldset" id="logplay-forward-fieldset">
    <a data-role="button" class="header-button" id="logplay-forward"><span><img src="./images/log_forward.png"
          title="Forward" /></span></a>
  </fieldset>
</div>
