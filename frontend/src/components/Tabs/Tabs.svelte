<style lang="scss">
  .tabs {
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    height: 200px;
  }

  a {
    background-color: grey;
    padding: 10px;
    border-radius: 0 10px 10px 0;
  }
  a img {
    height: 30px;
  }
</style>

<div class="tabs">
  <a id="mainMenuTab" class="gzGUI tab clickable">
    <img src="./images/bars.png" alt="Menu" title="Menu" />
  </a>
  <a id="treeMenuTab" class="gzGUI tab clickable">
    <img src="./images/tree.png" alt="Scene Tree" title="Scene Tree" />
  </a>
  <a id="insertMenuTab" class="gzGUI tab clickable">
    <img src="./images/insert.png" alt="Insert" title="Insert" />
  </a>
</div>
