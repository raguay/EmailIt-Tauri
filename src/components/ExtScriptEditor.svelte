<div id='ExtScriptEditor' style="color: {styles.textcolor}; left: {openPanel ? 6 : -470}px; background-color: {styles.appBackground};">
  {#if view === 'lists'}
    <div id='listviews'>
      <ExtScriptList
        config={config}
        styles={styles}
        ScriptPad={ScriptPad}
        on:changeView={changeView}
      />
      <EnvList
        config={config}
        styles={styles}
        ScriptPad={ScriptPad}
        on:changeView={changeView}
      />
    </div>
  {:else if view === 'script'}
    <ExtScript
      config={config}
      styles={styles}
      ScriptPad={ScriptPad}
      on:changeView={changeView}
    />
  {:else if view === 'env'}
    <Env
      config={config}
      styles={styles}
      ScriptPad={ScriptPad}
      on:changeView={changeView}
    />
  {/if}
  <div class='buttonRow'>
    <button 
      class="buttonStyle"
      style="background-color: {styles.editorBackground}; color: {styles.textcolor};"
      on:click={returnToMain}
    >
      Return
    </button>
  </div>
</div>

<style>
  #ExtScriptEditor {
    position: absolute;
    display: flex;
    flex-direction: column;
    padding: 5px;
    margin: 0px;
    width: 467px;
    height: 386px;
    top: 6px;
    z-index: 100;
    -webkit-transition: 0.75s ease-in-out;
    -moz-transition: 0.75s ease-in-out;
    -o-transition: 0.75s ease-in-out;
    transition: 0.75s ease-in-out;
  }

  #listviews {
    display: flex;
    flex-direction: column;
    width: 100%;
    padding: 0px;
    margin: 0px;
    overflow-y: scroll;
  }

  .buttonRow {
    display: flex;
    flex-direction: row;
    width: 100%;
    padding: 0px;
    margin: 20px 0px 0px 0px;
  }

  .buttonStyle {
    border-radius: 5px;
    border-color: black;
    font-size: 15px;
    height: 30px;
    text-shadow: 2px 2px 2px black;
    box-shadow: 2px 2px 5px 2px black;
    outline: none;
    margin: 0px 10px;
    padding: 3px 6px 6px 6px;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    -o-user-select: none;
    user-select: none;
    -webkit-tap-highlight-color:transparent;
    outline-style:none;
    cursor: pointer;
  }
</style>

<script>
  import { createEventDispatcher, onMount, tick  } from 'svelte';
  import ExtScriptList from './ExtScriptList.svelte';
  import ExtScript from './ExtScript.svelte';
  import EnvList from './EnvList.svelte';
  import Env from './Env.svelte';
  
  export let styles;
  export let mConfig;
  export let ScriptPad;
  export let openPanel;

  const dispatch = createEventDispatcher();
  
  let view = 'lists';
  let config = {};


  function changeView(newView) {
    view = newView.detail.view;
    config = newView.detail.config;
  }

  function returnToMain() {
    dispatch('returnToMain',{});
  }
</script>
