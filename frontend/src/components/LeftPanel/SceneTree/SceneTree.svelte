<div ng-controller="treeControl" class="panelsGroup">
  <div id="treeMenu" class="leftPanels ui-overlay-shadow">
    <div class="clickable panelTitle closePanels" title="Scene Tree">
      <h3>Scene Tree</h3>
    </div>
    <div class="panelContent">
      <!-- scene -->
      <Scene />
      <!-- physics -->
      <Physics />
      <!-- models -->
      <Models />
      <!-- lights -->
      <Lights />
    </div>
  </div>
  <!-- property panels -->
  <div ng-repeat="model in models" id="propertyPanel-{{model.id}}" class="leftPanels propertyPanels ui-overlay-shadow">
    <div class="clickable panelTitle" ng-click="openTab('treeMenu')" title="{{model.name}}">
      <h3>{{model.name}}</h3>
      <img src="style/images/back.png" />
    </div>
    <div class="panelContent">
      <div class="propertyPanelImg">
        <img src="{{model.thumbnail}}" />
      </div>
      <div class="propertyPanelHeader">
        <h4>Property</h4>
        <h4>Value</h4>
      </div>
      <!-- general -->
      <div id="expand-general-{{model.id}}" class="clickable expandableProperty expandableLevel1"
        ng-click="expandProperty('general', model.name)">
        <img src="style/images/play.png" />
        General
      </div>
      <div id="expandable-general-{{model.id}}" class="expandedContent">
        <div class="property propertyLevel2" title="{{model.name}}">
          <div>Name</div>
          <div>{{model.name}}</div>
        </div>
        <div class="property propertyLevel2">
          <div>Static</div>
          <div class="propertyToggle">
            <img src="style/images/{{model.is_static.icon}}.png" />
            <div>{{model.is_static.title}}</div>
          </div>
        </div>
        <!-- model pose -->
        <div id="expand-pose-{{model.id}}" class="clickable expandableProperty expandableLevel2"
          ng-click="expandProperty('pose', model.name)">
          <img src="style/images/play.png" />
          Pose
        </div>
        <div id="expandable-pose-{{model.id}}" class="expandedContent">
          <div class="property propertyLevel3 lightBg">
            <div>x</div>
            <div class="propertyNumberEdit">
              <input type="number" onClick="this.select();" ng-model="model.position.x"
                ng-change="changePose('position', 'x', model.name, model.position.x)" data-role="none" />
              <div class="propertyUnit">m</div>
            </div>
          </div>
          <div class="property propertyLevel3">
            <div>y</div>
            <div class="propertyNumberEdit">
              <input type="number" onClick="this.select();" ng-model="model.position.y"
                ng-change="changePose('position', 'y', model.name, model.position.y)" data-role="none" />
              <div class="propertyUnit">m</div>
            </div>
          </div>
          <div class="property propertyLevel3 lightBg">
            <div>z</div>
            <div class="propertyNumberEdit">
              <input type="number" onClick="this.select();" ng-model="model.position.z"
                ng-change="changePose('position', 'z', model.name, model.position.z)" data-role="none" />
              <div class="propertyUnit">m</div>
            </div>
          </div>
          <div class="property propertyLevel3">
            <div>Roll</div>
            <div class="propertyNumberEdit">
              <input type="number" onClick="this.select();" ng-model="model.orientation.roll"
                ng-change="changePose('orientation', 'x', model.name, model.orientation.roll)" data-role="none" />
              <div class="propertyUnit propertyUnit3">rad</div>
            </div>
          </div>
          <div class="property propertyLevel3 lightBg">
            <div>Pitch</div>
            <div class="propertyNumberEdit">
              <input type="number" onClick="this.select();" ng-model="model.orientation.pitch"
                ng-change="changePose('orientation', 'y', model.name, model.orientation.pitch)" data-role="none" />
              <div class="propertyUnit propertyUnit3">rad</div>
            </div>
          </div>
          <div class="property propertyLevel3">
            <div>Yaw</div>
            <div class="propertyNumberEdit">
              <input type="number" onClick="this.select();" ng-model="model.orientation.yaw"
                ng-change="changePose('orientation', 'z', model.name, model.orientation.yaw)" data-role="none" />
              <div class="propertyUnit propertyUnit3">rad</div>
            </div>
          </div>
        </div>
      </div>
      <!-- links -->
      <div ng-if="model.links" id="expand-links-{{model.id}}" class="clickable expandableProperty expandableLevel1"
        ng-click="expandProperty('links', model.name)">
        <img src="style/images/play.png" />
        Links
      </div>
      <div id="expandable-links-{{model.id}}" class="expandedContent">
        <div ng-repeat="link in model.links">
          <div id="expand-link-{{model.id}}-{{link.shortName}}" class="clickable expandableProperty expandableLevel2"
            ng-click="expandProperty('link', model.name, link.shortName)" title="{{link.name}}">
            <img src="style/images/play.png" />
            {{link.shortName}}
          </div>
          <div id="expandable-link-{{model.id}}-{{link.shortName}}" class="expandedContent">
            <div class="property propertyLevel3" title="{{link.name}}">
              <div>Name</div>
              <div>{{link.name}}</div>
            </div>
            <div class="clickable property propertyLevel3" ng-click="toggleProperty('self_collide', link.name)">
              <div>Self Collide</div>
              <div class="propertyToggle">
                <img src="style/images/{{link.self_collide.icon}}.png" />
                <div>{{link.self_collide.title}}</div>
              </div>
            </div>
            <div class="clickable property propertyLevel3" ng-click="toggleProperty('gravity', link.name)">
              <div>Gravity</div>
              <div class="propertyToggle">
                <img src="style/images/{{link.gravity.icon}}.png" />
                <div>{{link.gravity.title}}</div>
              </div>
            </div>
            <div class="clickable property propertyLevel3" ng-click="toggleProperty('kinematic', link.name)">
              <div>Kinematic</div>
              <div class="propertyToggle">
                <img src="style/images/{{link.kinematic.icon}}.png" />
                <div>{{link.kinematic.title}}</div>
              </div>
            </div>
            <div class="property propertyLevel3">
              <div>Canonical</div>
              <div class="propertyToggle">
                <img src="style/images/{{link.canonical.icon}}.png" />
                <div>{{link.canonical.title}}</div>
              </div>
            </div>
            <!-- link pose -->
            <div id="expand-pose-{{model.id}}-{{link.shortName}}" class="clickable expandableProperty expandableLevel3"
              ng-click="expandProperty('pose', model.name, link.shortName, link.name, 'link')">
              <img src="style/images/play.png" />
              Pose
            </div>
            <div id="expandable-pose-{{model.id}}-{{link.shortName}}" class="expandedContent">
              <div class="property propertyLevel4 lightBg">
                <div>x</div>
                <div class="propertyNumber">
                  <div>{{link.position.x}}</div>
                  <div class="propertyUnit">m</div>
                </div>
              </div>
              <div class="property propertyLevel4">
                <div>y</div>
                <div class="propertyNumber">
                  <div>{{link.position.y}}</div>
                  <div class="propertyUnit">m</div>
                </div>
              </div>
              <div class="property propertyLevel4 lightBg">
                <div>z</div>
                <div class="propertyNumber">
                  <div>{{link.position.z}}</div>
                  <div class="propertyUnit">m</div>
                </div>
              </div>
              <div class="property propertyLevel4">
                <div>Roll</div>
                <div class="propertyNumber">
                  <div>{{link.orientation.roll}}</div>
                  <div class="propertyUnit propertyUnit3">rad</div>
                </div>
              </div>
              <div class="property propertyLevel4 lightBg">
                <div>Pitch</div>
                <div class="propertyNumber">
                  <div>{{link.orientation.pitch}}</div>
                  <div class="propertyUnit propertyUnit3">rad</div>
                </div>
              </div>
              <div class="property propertyLevel4">
                <div>Yaw</div>
                <div class="propertyNumber">
                  <div>{{link.orientation.yaw}}</div>
                  <div class="propertyUnit propertyUnit3">rad</div>
                </div>
              </div>
            </div>
            <!-- link inertial -->
            <div id="expand-inertial-{{model.id}}-{{link.shortName}}"
              class="clickable expandableProperty expandableLevel3"
              ng-click="expandProperty('inertial', model.name, link.shortName)">
              <img src="style/images/play.png" />
              Inertial
            </div>
            <div id="expandable-inertial-{{model.id}}-{{link.shortName}}" class="expandedContent">
              <div class="property propertyLevel4">
                <div>Mass</div>
                <div class="propertyNumber">
                  <div>{{link.inertial.mass}}</div>
                  <div class="propertyUnit propertyUnit3">Kg</div>
                </div>
              </div>
              <!-- link inertial pose -->
              <div id="expand-inertial-pose-{{model.id}}-{{link.shortName}}"
                class="clickable expandableProperty expandableLevel4"
                ng-click="expandProperty('inertial-pose', model.name, link.shortName)">
                <img src="style/images/play.png" />
                Pose
              </div>
              <div id="expandable-inertial-pose-{{model.id}}-{{link.shortName}}" class="expandedContent">
                <div class="property propertyLevel5 lightBg">
                  <div>x</div>
                  <div class="propertyNumber">
                    <div>{{link.inertial.pose.position.x}}</div>
                    <div class="propertyUnit">m</div>
                  </div>
                </div>
                <div class="property propertyLevel5">
                  <div>y</div>
                  <div class="propertyNumber">
                    <div>{{link.inertial.pose.position.y}}</div>
                    <div class="propertyUnit">m</div>
                  </div>
                </div>
                <div class="property propertyLevel5 lightBg">
                  <div>z</div>
                  <div class="propertyNumber">
                    <div>{{link.inertial.pose.position.z}}</div>
                    <div class="propertyUnit">m</div>
                  </div>
                </div>
                <div class="property propertyLevel5">
                  <div>Roll</div>
                  <div class="propertyNumber">
                    <div>{{link.inertial.pose.orientation.roll}}</div>
                    <div class="propertyUnit propertyUnit3">rad</div>
                  </div>
                </div>
                <div class="property propertyLevel5 lightBg">
                  <div>Pitch</div>
                  <div class="propertyNumber">
                    <div>{{link.inertial.pose.orientation.pitch}}</div>
                    <div class="propertyUnit propertyUnit3">rad</div>
                  </div>
                </div>
                <div class="property propertyLevel5">
                  <div>Yaw</div>
                  <div class="propertyNumber">
                    <div>{{link.inertial.pose.orientation.yaw}}</div>
                    <div class="propertyUnit propertyUnit3">rad</div>
                  </div>
                </div>
              </div>
              <!-- link inertial moments -->
              <div id="expand-inertial-moments-{{model.id}}-{{link.shortName}}"
                class="clickable expandableProperty expandableLevel4"
                ng-click="expandProperty('inertial-moments', model.name, link.shortName)">
                <img src="style/images/play.png" />
                Moments
              </div>
              <div id="expandable-inertial-moments-{{model.id}}-{{link.shortName}}" class="expandedContent">
                <div class="property propertyLevel5 lightBg">
                  <div>ixx</div>
                  <div class="propertyNumber">
                    <div>{{link.inertial.ixx}}</div>
                    <div class="propertyUnit propertyUnit4">
                      Kg.m<sup>2</sup>
                    </div>
                  </div>
                </div>
                <div class="property propertyLevel5">
                  <div>ixy</div>
                  <div class="propertyNumber">
                    <div>{{link.inertial.ixy}}</div>
                    <div class="propertyUnit propertyUnit4">
                      Kg.m<sup>2</sup>
                    </div>
                  </div>
                </div>
                <div class="property propertyLevel5 lightBg">
                  <div>ixz</div>
                  <div class="propertyNumber">
                    <div>{{link.inertial.ixz}}</div>
                    <div class="propertyUnit propertyUnit4">
                      Kg.m<sup>2</sup>
                    </div>
                  </div>
                </div>
                <div class="property propertyLevel5">
                  <div>iyy</div>
                  <div class="propertyNumber">
                    <div>{{link.inertial.iyy}}</div>
                    <div class="propertyUnit propertyUnit4">
                      Kg.m<sup>2</sup>
                    </div>
                  </div>
                </div>
                <div class="property propertyLevel5 lightBg">
                  <div>iyz</div>
                  <div class="propertyNumber">
                    <div>{{link.inertial.iyz}}</div>
                    <div class="propertyUnit propertyUnit4">
                      Kg.m<sup>2</sup>
                    </div>
                  </div>
                </div>
                <div class="property propertyLevel5">
                  <div>izz</div>
                  <div class="propertyNumber">
                    <div>{{link.inertial.izz}}</div>
                    <div class="propertyUnit propertyUnit4">
                      Kg.m<sup>2</sup>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <!-- joints -->
      <div ng-if="model.joints" id="expand-joints-{{model.id}}" class="clickable expandableProperty expandableLevel1"
        ng-click="expandProperty('joints', model.name)">
        <img src="style/images/play.png" />
        Joints
      </div>
      <div id="expandable-joints-{{model.id}}" class="expandedContent">
        <div ng-repeat="joint in model.joints">
          <div id="expand-joint-{{model.id}}-{{joint.shortName}}" class="clickable expandableProperty expandableLevel2"
            ng-click="expandProperty('joint', model.name, joint.shortName)" title="{{joint.name}}">
            <img src="style/images/play.png" />
            {{joint.shortName}}
          </div>
          <div id="expandable-joint-{{model.id}}-{{joint.shortName}}" class="expandedContent">
            <div class="property propertyLevel3" title="{{joint.name}}">
              <div>Name</div>
              <div>{{joint.name}}</div>
            </div>
            <div class="property propertyLevel3" title="{{joint.type}}">
              <div>Type</div>
              <div>{{joint.type}}</div>
            </div>
            <div class="property propertyLevel3" title="{{joint.parent}}">
              <div>Parent Link</div>
              <div>{{joint.parentShortName}}</div>
            </div>
            <div class="property propertyLevel3" title="{{joint.child}}">
              <div>Child Link</div>
              <div>{{joint.childShortName}}</div>
            </div>
            <!-- joint pose -->
            <div id="expand-pose-{{model.id}}-{{joint.shortName}}" class="clickable expandableProperty expandableLevel3"
              ng-click="expandProperty('pose', model.name, joint.shortName, joint.name, 'joint')">
              <img src="style/images/play.png" />
              Pose
            </div>
            <div id="expandable-pose-{{model.id}}-{{joint.shortName}}" class="expandedContent">
              <div class="property propertyLevel4 lightBg">
                <div>x</div>
                <div class="propertyNumber">{{joint.position.x}}</div>
              </div>
              <div class="property propertyLevel4">
                <div>y</div>
                <div class="propertyNumber">{{joint.position.y}}</div>
              </div>
              <div class="property propertyLevel4 lightBg">
                <div>z</div>
                <div class="propertyNumber">{{joint.position.z}}</div>
              </div>
              <div class="property propertyLevel4">
                <div>Roll</div>
                <div class="propertyNumber">
                  {{joint.orientation.roll}}
                </div>
              </div>
              <div class="property propertyLevel4 lightBg">
                <div>Pitch</div>
                <div class="propertyNumber">
                  {{joint.orientation.pitch}}
                </div>
              </div>
              <div class="property propertyLevel4">
                <div>Yaw</div>
                <div class="propertyNumber">
                  {{joint.orientation.yaw}}
                </div>
              </div>
            </div>
            <!-- joint axis 1 -->
            <div ng-if="joint.axis1" id="expand-axis1-{{model.id}}-{{joint.shortName}}"
              class="clickable expandableProperty expandableLevel3"
              ng-click="expandProperty('axis1', model.name, joint.shortName, joint.name, 'joint')">
              <img src="style/images/play.png" />
              Axis 1
            </div>
            <div id="expandable-axis1-{{model.id}}-{{joint.shortName}}" class="expandedContent">
              <div id="expand-axis1-direction-{{model.id}}-{{joint.shortName}}"
                class="clickable expandableProperty expandableLevel4"
                ng-click="expandProperty('axis1-direction', model.name, joint.shortName, joint.name, 'joint')">
                <img src="style/images/play.png" />
                Direction
              </div>
              <div id="expandable-axis1-direction-{{model.id}}-{{joint.shortName}}" class="expandedContent">
                <div class="property propertyLevel5 lightBg">
                  <div>x</div>
                  <div class="propertyNumber">
                    {{joint.axis1.direction.x}}
                  </div>
                </div>
                <div class="property propertyLevel5">
                  <div>y</div>
                  <div class="propertyNumber">
                    {{joint.axis1.direction.y}}
                  </div>
                </div>
                <div class="property propertyLevel5 lightBg">
                  <div>z</div>
                  <div class="propertyNumber">
                    {{joint.axis1.direction.z}}
                  </div>
                </div>
              </div>
              <div class="property propertyLevel4">
                <div>Lower Limit</div>
                <div class="propertyNumber">
                  {{joint.axis1.limit_lower}}
                </div>
              </div>
              <div class="property propertyLevel4">
                <div>Upper Limit</div>
                <div class="propertyNumber">
                  {{joint.axis1.limit_upper}}
                </div>
              </div>
              <div class="property propertyLevel4">
                <div>Effort</div>
                <div class="propertyNumber">
                  {{joint.axis1.limit_effort}}
                </div>
              </div>
              <div class="property propertyLevel4">
                <div>Velocity</div>
                <div class="propertyNumber">
                  {{joint.axis1.limit_velocity}}
                </div>
              </div>
              <div class="property propertyLevel4">
                <div>Damping</div>
                <div class="propertyNumber">
                  {{joint.axis1.damping}}
                </div>
              </div>
              <div class="property propertyLevel4">
                <div>Friction</div>
                <div class="propertyNumber">
                  {{joint.axis1.friction}}
                </div>
              </div>
            </div>
            <!-- joint axis 2 -->
            <div ng-if="joint.axis2" id="expand-axis2-{{model.id}}-{{joint.shortName}}"
              class="clickable expandableProperty expandableLevel3"
              ng-click="expandProperty('axis2', model.name, joint.shortName, joint.name, 'joint')">
              <img src="style/images/play.png" />
              Axis 2
            </div>
            <div id="expandable-axis2-{{model.id}}-{{joint.shortName}}" class="expandedContent">
              <div id="expand-axis2-direction-{{model.id}}-{{joint.shortName}}"
                class="clickable expandableProperty expandableLevel4"
                ng-click="expandProperty('axis2-direction', model.name, joint.shortName, joint.name, 'joint')">
                <img src="style/images/play.png" />
                Direction
              </div>
              <div id="expandable-axis2-direction-{{model.id}}-{{joint.shortName}}" class="expandedContent">
                <div class="property propertyLevel5 lightBg">
                  <div>x</div>
                  <div class="propertyNumber">
                    {{joint.axis2.direction.x}}
                  </div>
                </div>
                <div class="property propertyLevel5">
                  <div>y</div>
                  <div class="propertyNumber">
                    {{joint.axis2.direction.y}}
                  </div>
                </div>
                <div class="property propertyLevel5 lightBg">
                  <div>z</div>
                  <div class="propertyNumber">
                    {{joint.axis2.direction.z}}
                  </div>
                </div>
              </div>
              <div class="property propertyLevel4">
                <div>Lower</div>
                <div class="propertyNumber">
                  {{joint.axis2.limit_lower}}
                </div>
              </div>
              <div class="property propertyLevel4">
                <div>Upper</div>
                <div class="propertyNumber">
                  {{joint.axis2.limit_upper}}
                </div>
              </div>
              <div class="property propertyLevel4">
                <div>Effort</div>
                <div class="propertyNumber">
                  {{joint.axis2.limit_effort}}
                </div>
              </div>
              <div class="property propertyLevel4">
                <div>Velocity</div>
                <div class="propertyNumber">
                  {{joint.axis2.limit_velocity}}
                </div>
              </div>
              <div class="property propertyLevel4">
                <div>Damping</div>
                <div class="propertyNumber">
                  {{joint.axis2.damping}}
                </div>
              </div>
              <div class="property propertyLevel4">
                <div>Friction</div>
                <div class="propertyNumber">
                  {{joint.axis2.friction}}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <!-- lights -->
  <div ng-repeat="light in lights" id="propertyPanel-{{light.id}}" class="leftPanels propertyPanels ui-overlay-shadow">
    <div class="clickable panelTitle" ng-click="openTab('treeMenu')" title="{{light.name}}">
      <h3>{{light.name}}</h3>
      <img src="style/images/back.png" />
    </div>
    <div class="panelContent">
      <div class="propertyPanelImg">
        <img src="{{light.thumbnail}}" />
      </div>
      <div class="propertyPanelHeader">
        <h4>Property</h4>
        <h4>Value</h4>
      </div>
      <div class="property propertyLevel1" title="{{light.name}}">
        <div>Name</div>
        <div>{{light.name}}</div>
      </div>
      <!-- light pose -->
      <div id="expand-pose-{{light.id}}" class="clickable expandableProperty"
        ng-click="expandProperty('pose', light.name)">
        <img src="style/images/play.png" />
        Pose
      </div>
      <div id="expandable-pose-{{light.id}}" class="expandedContent">
        <div class="property propertyLevel2 lightBg">
          <div>x</div>
          <div class="propertyNumberEdit">
            <input type="number" onClick="this.select();" ng-model="light.position.x"
              ng-change="changePose('position', 'x', light.name, light.position.x)" data-role="none" />
            <div class="propertyUnit">m</div>
          </div>
        </div>
        <div class="property propertyLevel2">
          <div>y</div>
          <div class="propertyNumberEdit">
            <input type="number" onClick="this.select();" ng-model="light.position.y"
              ng-change="changePose('position', 'y', light.name, light.position.y)" data-role="none" />
            <div class="propertyUnit">m</div>
          </div>
        </div>
        <div class="property propertyLevel2 lightBg">
          <div>z</div>
          <div class="propertyNumberEdit">
            <input type="number" onClick="this.select();" ng-model="light.position.z"
              ng-change="changePose('position', 'z', light.name, light.position.z)" data-role="none" />
            <div class="propertyUnit">m</div>
          </div>
        </div>
        <div class="property propertyLevel2">
          <div>Roll</div>
          <div class="propertyNumberEdit">
            <input type="number" onClick="this.select();" ng-model="light.orientation.roll"
              ng-change="changePose('orientation', 'x', light.name, light.orientation.roll)" data-role="none" />
            <div class="propertyUnit propertyUnit3">rad</div>
          </div>
        </div>
        <div class="property propertyLevel2 lightBg">
          <div>Pitch</div>
          <div class="propertyNumberEdit">
            <input type="number" onClick="this.select();" ng-model="light.orientation.pitch"
              ng-change="changePose('orientation', 'y', light.name, light.orientation.pitch)" data-role="none" />
            <div class="propertyUnit propertyUnit3">rad</div>
          </div>
        </div>
        <div class="property propertyLevel2">
          <div>Yaw</div>
          <div class="propertyNumberEdit">
            <input type="number" onClick="this.select();" ng-model="light.orientation.yaw"
              ng-change="changePose('orientation', 'z', light.name, light.orientation.yaw)" data-role="none" />
            <div class="propertyUnit propertyUnit3">rad</div>
          </div>
        </div>
      </div>
      <!-- light direction -->
      <div ng-if="light.direction" id="expand-direction-{{light.id}}" class="clickable expandableProperty"
        ng-click="expandProperty('direction', light.name)">
        <img src="style/images/play.png" />
        Direction
      </div>
      <div id="expandable-direction-{{light.id}}" class="expandedContent">
        <div class="property propertyLevel2 lightBg">
          <div>x</div>
          <div class="propertyNumberEdit">
            <input type="number" onClick="this.select();" ng-model="light.direction.x"
              ng-change="changePose('direction', 'x', light.name, light.direction.x)" data-role="none" />
          </div>
        </div>
        <div class="property propertyLevel2">
          <div>y</div>
          <div class="propertyNumberEdit">
            <input type="number" onClick="this.select();" ng-model="light.direction.y"
              ng-change="changePose('direction', 'y', light.name, light.direction.y)" data-role="none" />
          </div>
        </div>
        <div class="property propertyLevel2 lightBg">
          <div>z</div>
          <div class="propertyNumberEdit">
            <input type="number" onClick="this.select();" ng-model="light.direction.z"
              ng-change="changePose('direction', 'z', light.name, light.direction.z)" data-role="none" />
          </div>
        </div>
      </div>
      <!-- light diffuse -->
      <div id="expand-diffuse-{{light.id}}" class="clickable expandableProperty"
        ng-click="expandProperty('diffuse', light.name)">
        <div>
          <img src="style/images/play.png" />
          Diffuse
        </div>
        <div class="propertyColorEdit">
          <input class="clickable" type="color" ng-model="light.color.diffuse"
            ng-change="changeLight('diffuse', light.name, light.color.diffuse)" />
        </div>
      </div>
      <div id="expandable-diffuse-{{light.id}}" class="expandedContent">
        <div class="property propertyLevel2 redBg">
          <div>Red</div>
          <div class="propertyNumber">{{light.diffuse.r}}</div>
        </div>
        <div class="property propertyLevel2 greenBg">
          <div>Green</div>
          <div class="propertyNumber">{{light.diffuse.g}}</div>
        </div>
        <div class="property propertyLevel2 blueBg">
          <div>Blue</div>
          <div class="propertyNumber">{{light.diffuse.b}}</div>
        </div>
        <div class="property propertyLevel2">
          <div>Alpha</div>
          <div class="propertyNumber">{{light.diffuse.a}}</div>
        </div>
      </div>
      <!-- light specular -->
      <div id="expand-specular-{{light.id}}" class="clickable expandableProperty"
        ng-click="expandProperty('specular', light.name)">
        <div>
          <img src="style/images/play.png" />
          Specular
        </div>
        <div class="propertyColorEdit">
          <input class="clickable" type="color" ng-model="light.color.specular"
            ng-change="changeLight('specular', light.name, light.color.specular)" />
        </div>
      </div>
      <div id="expandable-specular-{{light.id}}" class="expandedContent">
        <div class="property propertyLevel2 redBg">
          <div>Red</div>
          <div class="propertyNumber">{{light.specular.r}}</div>
        </div>
        <div class="property propertyLevel2 greenBg">
          <div>Green</div>
          <div class="propertyNumber">{{light.specular.g}}</div>
        </div>
        <div class="property propertyLevel2 blueBg">
          <div>Blue</div>
          <div class="propertyNumber">{{light.specular.b}}</div>
        </div>
        <div class="property propertyLevel2">
          <div>Alpha</div>
          <div class="propertyNumber">{{light.specular.a}}</div>
        </div>
      </div>
      <div class="property propertyLevel1">
        <div>Range</div>
        <div class="propertyNumberEdit">
          <input type="number" onClick="this.select();" ng-model="light.range"
            ng-change="changeLight('range', light.name, light.range)" data-role="none" />
        </div>
      </div>
      <!-- light attenuation -->
      <div id="expand-attenuation-{{light.id}}" class="clickable expandableProperty"
        ng-click="expandProperty('attenuation', light.name)">
        <img src="style/images/play.png" />
        Attenuation
      </div>
      <div id="expandable-attenuation-{{light.id}}" class="expandedContent">
        <div class="property propertyLevel2">
          <div>Constant</div>
          <div class="propertyNumberEdit">
            <input type="number" onClick="this.select();" ng-model="light.attenuation.constant"
              ng-change="changeLight('attenuation_constant', light.name, light.attenuation.constant)"
              data-role="none" />
          </div>
        </div>
        <div class="property propertyLevel2">
          <div>Linear</div>
          <div class="propertyNumberEdit">
            <input type="number" onClick="this.select();" ng-model="light.attenuation.linear"
              ng-change="changeLight('attenuation_linear', light.name, light.attenuation.linear)" data-role="none" />
          </div>
        </div>
        <div class="property propertyLevel2">
          <div>Quadratic</div>
          <div class="propertyNumberEdit">
            <input type="number" onClick="this.select();" ng-model="light.attenuation.quadratic"
              ng-change="changeLight('attenuation_quadratic', light.name, light.attenuation.quadratic)"
              data-role="none" />
          </div>
        </div>
      </div>
    </div>
  </div>
  <!-- scene -->
  <div id="sceneProperties" class="leftPanels propertyPanels ui-overlay-shadow">
    <div class="clickable panelTitle" ng-click="openTab('treeMenu')" title="Scene">
      <h3>Scene</h3>
      <img src="style/images/back.png" />
    </div>
    <div class="panelContent">
      <div class="propertyPanelHeader">
        <h4>Property</h4>
        <h4>Value</h4>
      </div>
      <!-- scene ambient -->
      <div id="expand-ambient-SCENE" class="clickable expandableProperty expandableLevel1"
        ng-click="expandProperty('ambient', 'SCENE')">
        <div>
          <img src="style/images/play.png" />
          Ambient
        </div>
        <div class="colorDisplay"
          style="background-color: rgb({{scene.ambient.r}},{{scene.ambient.g}},{{scene.ambient.b}})"></div>
      </div>
      <div id="expandable-ambient-SCENE" class="expandedContent">
        <div class="property propertyLevel2 redBg">
          <div>Red</div>
          <div class="propertyNumber">{{scene.ambient.r}}</div>
        </div>
        <div class="property propertyLevel2 greenBg">
          <div>Green</div>
          <div class="propertyNumber">{{scene.ambient.g}}</div>
        </div>
        <div class="property propertyLevel2 blueBg">
          <div>Blue</div>
          <div class="propertyNumber">{{scene.ambient.b}}</div>
        </div>
        <div class="property propertyLevel2">
          <div>Alpha</div>
          <div class="propertyNumber">{{scene.ambient.a}}</div>
        </div>
      </div>
      <!-- scene background -->
      <div id="expand-background-SCENE" class="clickable expandableProperty expandableLevel1"
        ng-click="expandProperty('background', 'SCENE')">
        <div>
          <img src="style/images/play.png" />
          Background
        </div>
        <div class="colorDisplay"
          style="background-color: rgb({{scene.background.r}},{{scene.background.g}},{{scene.background.b}})"></div>
      </div>
      <div id="expandable-background-SCENE" class="expandedContent">
        <div class="property propertyLevel2 redBg">
          <div>Red</div>
          <div class="propertyNumber">{{scene.background.r}}</div>
        </div>
        <div class="property propertyLevel2 greenBg">
          <div>Green</div>
          <div class="propertyNumber">{{scene.background.g}}</div>
        </div>
        <div class="property propertyLevel2 blueBg">
          <div>Blue</div>
          <div class="propertyNumber">{{scene.background.b}}</div>
        </div>
        <div class="property propertyLevel2">
          <div>Alpha</div>
          <div class="propertyNumber">{{scene.background.a}}</div>
        </div>
      </div>
    </div>
  </div>
  <!-- physics -->
  <div id="physicsProperties" class="leftPanels propertyPanels ui-overlay-shadow">
    <div class="clickable panelTitle" ng-click="openTab('treeMenu')" title="Physics">
      <h3>Physics</h3>
      <img src="style/images/back.png" />
    </div>
    <div class="panelContent">
      <div class="propertyPanelHeader">
        <h4>Property</h4>
        <h4>Value</h4>
      </div>
      <div class="property propertyLevel1">
        <div>Enable Physics</div>
        <div class="propertyToggle">
          <img src="style/images/{{physics.enable_physics.icon}}.png" />
          <div>{{physics.enable_physics.title}}</div>
        </div>
      </div>
      <div class="property propertyLevel1">
        <div id="real-time-update-rate">Real Time Update Rate</div>
        <div class="propertyNumber">
          {{physics.real_time_update_rate}}
        </div>
      </div>
      <div class="property propertyLevel1">
        <div>Max Step Size</div>
        <div class="propertyNumber">{{physics.max_step_size}}</div>
      </div>
      <div id="expand-gravity-PHYSICS" class="clickable expandableProperty expandableLevel1"
        ng-click="expandProperty('gravity', 'PHYSICS')">
        <div>
          <img src="style/images/play.png" />
          Gravity
        </div>
      </div>
      <div id="expandable-gravity-PHYSICS" class="expandedContent">
        <div class="property propertyLevel2 lightBg">
          <div>x</div>
          <div class="propertyNumber">{{physics.gravity.x}}</div>
        </div>
        <div class="property propertyLevel2">
          <div>y</div>
          <div class="propertyNumber">{{physics.gravity.y}}</div>
        </div>
        <div class="property propertyLevel2 lightBg">
          <div>z</div>
          <div class="propertyNumber">{{physics.gravity.z}}</div>
        </div>
      </div>
      <div id="expand-solver-PHYSICS" class="clickable expandableProperty expandableLevel1"
        ng-click="expandProperty('solver', 'PHYSICS')">
        <div>
          <img src="style/images/play.png" />
          Solver
        </div>
      </div>
      <div id="expandable-solver-PHYSICS" class="expandedContent">
        <div class="property propertyLevel2">
          <div>Iterations</div>
          <div class="propertyNumber">{{physics.iters}}</div>
        </div>
        <div class="property propertyLevel2">
          <div>SOR</div>
          <div class="propertyNumber">{{physics.sor}}</div>
        </div>
      </div>
      <div id="expand-constraints-PHYSICS" class="clickable expandableProperty expandableLevel1"
        ng-click="expandProperty('constraints', 'PHYSICS')">
        <div>
          <img src="style/images/play.png" />
          Constraints
        </div>
      </div>
      <div id="expandable-constraints-PHYSICS" class="expandedContent">
        <div class="property propertyLevel2 lightBg">
          <div>CFM</div>
          <div class="propertyNumber">{{physics.cfm}}</div>
        </div>
        <div class="property propertyLevel2">
          <div>ERP</div>
          <div class="propertyNumber">{{physics.erp}}</div>
        </div>
        <div class="property propertyLevel2 lightBg">
          <div>Max Velocity</div>
          <div class="propertyNumber">
            {{physics.contact_max_correcting_vel}}
          </div>
        </div>
        <div class="property propertyLevel2">
          <div>Surface Layer</div>
          <div class="propertyNumber">
            {{physics.contact_surface_layer}}
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
