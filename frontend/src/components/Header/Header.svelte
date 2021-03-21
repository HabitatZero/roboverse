<fieldset class="gzGUI" id="play-header-fieldset">
  <a data-role="button" class="header-button" id="play"><span id="playText"><img src="style/images/play.png"
        title="Play" /></span></a>
</fieldset>

<fieldset id="clock-header-fieldset" class="gzGUI touch-only">
  <a data-role="button" id="clock" class="header-button" data-rel="popup"><span><img
        src="style/images/clock.png" /></span></a>
</fieldset>

<fieldset id="clock-mouse" class="gzGUI mouse-only">
  <p>Real Time: <span class="real-time-value">00:00:00.000</span></p>
  <p>Sim Time: <span class="sim-time-value">00:00:00.000</span></p>
</fieldset>

<div class="gzGUI" id="clock-touch" data-role="popup" data-history="false">
  <p>Real Time: <span class="real-time-value">00:00:00.000</span></p>
  <p>Sim Time: <span class="sim-time-value">00:00:00.000</span></p>
</div>

<div id="manipulatorModeFieldset">
  <fieldset class="gzGUI" data-role="controlgroup" data-mini="true" data-type="horizontal" id="mode-header-fieldset">
    <input type="radio" name="radio-mini" id="view-mode" value="choice-1" checked="checked" />
    <label for="view-mode"><img style="height: 1.8em" src="style/images/arrow.png" title="View Mode (ESC)" /></label>

    <input type="radio" name="radio-mini" id="translate-mode" value="choice-2" />
    <label for="translate-mode"><img style="height: 1.8em" src="style/images/translate.png"
        title="Translation Mode (T)" /></label>

    <input type="radio" name="radio-mini" id="rotate-mode" value="choice-3" />
    <label for="rotate-mode"><img style="height: 1.8em" src="style/images/rotate.png"
        title="Rotation Mode (R)" /></label>
  </fieldset>
</div>

<div id="simpleShapesFieldset">
  <fieldset id="box-header-fieldset" class="gzGUI mouse-only">
    <a data-role="button" id="header-insert-box" class="header-button"><span><img src="style/images/box.png"
          title="Insert box" /></span></a>
  </fieldset>
  <fieldset id="sphere-header-fieldset" class="gzGUI mouse-only">
    <a data-role="button" id="header-insert-sphere" class="header-button"><span><img src="style/images/sphere.png"
          title="Insert sphere" /></span></a>
  </fieldset>
  <fieldset id="cylinder-header-fieldset" class="gzGUI mouse-only">
    <a data-role="button" id="header-insert-cylinder" class="header-button"><span><img src="style/images/cylinder.png"
          title="Insert cylinder" /></span></a>
  </fieldset>
</div>

<div id="lightsFieldset">
  <fieldset id="pointlight-header-fieldset" class="gzGUI mouse-only">
    <a data-role="button" id="header-insert-pointlight" class="header-button"><span><img style="height: 1.45em"
          src="style/images/pointlight.png" title="Insert point light" /></span></a>
  </fieldset>
  <fieldset id="spotlight-header-fieldset" class="gzGUI mouse-only">
    <a data-role="button" id="header-insert-spotlight" class="header-button"><span><img src="style/images/spotlight.png"
          title="Insert spot light" /></span></a>
  </fieldset>
  <fieldset id="directionallight-header-fieldset" class="gzGUI mouse-only">
    <a data-role="button" id="header-insert-directionallight" class="header-button"><span><img
          src="style/images/directionallight.png" title="Insert directional light" /></span></a>
  </fieldset>
</div>
