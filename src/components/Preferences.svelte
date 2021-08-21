<div id="PreferencePanel" class="{openPanel ? 'visible' : 'invisible'}" style="background-color: {styles.editorBackground};">
  <ul id="tabs">
    {#each prefs as pref}
      {#if pref.name === currentPref}
        <li class="tabName current">{pref.name}</li>
      {:else}
        <li class="tabName notCurrent" on:click="{(event) => { setNewCurrent(pref.name);}}">{pref.name}</li>
      {/if}
    {/each}
  </ul>
  <div id="prefListWrap">
    {#each prefs as pref,key1}
      {#if pref.name === currentPref}
        <div class="prefListPanel">
          {#if pref.name === 'General'}
            <h2>General</h2>
            <div class="labelRadialHolder">
              <label class="styleLabel">Use line numbers in the code editor?</label>
              <input class="PrefCheck" type="checkbox" bind:checked={preferences.codeLineNumbers} />
            </div>
            <div class="labelRadialHolder">
              <label class="styleLabel">Use line numbers in the note editor?</label>                            
              <input class="PrefCheck" type="checkbox" bind:checked={preferences.noteLineNumbers} />
            </div>
            <div class="labelRadialHolder">
              <label class="styleLabel">Note Editor KeyMap:</label>                            
              <select class="prefSelector" style="background-color: {styles.editorBackground}; color: {styles.textcolor};" bind:value={preferences.noteKeyMap} >
                <option value="sublime">Sublime</option>
                <option value="vim">Vim</option>
                <option value="emacs">Emacs</option>
              </select>
            </div>
            <div class="labelRadialHolder">
              <label class="styleLabel">Code Editor KeyMap:</label>
              <select class="prefSelector" style="background-color: {styles.editorBackground}; color: {styles.textcolor};" bind:value={preferences.scriptKeyMap} >
                <option value="sublime">Sublime</option>
                <option value="vim">Vim</option>
                <option value="emacs">Emacs</option>
              </select>
            </div>
            <div class="labelRadialHolder">
              <label class="styleLabel">Clear Cache on Exit?</label>                            
              <input class="PrefCheck" type="checkbox" bind:checked={preferences.clearCache} />
            </div>
          {:else if pref.name === 'Theme'}
            <h2>Theme</h2>
            <div id="themeName">
              <label>Name of theme: </label>
              {#if (preferences.style === 'New')||(keepNewInput)}
                <input class="prefInput" style="background-color: {styles.editorBackground}; color: {styles.textcolor};" bind:value={preferences.style}>
              {:else}
                <select class="prefSelector" style="background-color: {styles.editorBackground}; color: {styles.textcolor};" bind:value={preferences.style} on:change="{(event) => {styleSelectorChange()}}">
                {#each getStyleList() as item}
                  <option value={item}>{item}</option>
                {/each}
                </select>
              {/if}
            </div>
            <h3>Circle Colors</h3>
            <div class="circlePickersWrap">
              {#each styles.buttons as button, key}
                <div class="circlePickerWrap">
                  <label class="circlePickerLabel">#{key + 1}</label>
                  <div class="circlePicker" on:click="{(event) => {changeColor(event, button);}}" style="background-color: {button.color};" >
                  </div>
                  <label class="circlePickerLabel">{button.color}</label>
                </div>
              {/each}
            </div>
            <h3>Regular Expression Sub-Matches Colors</h3>
            <div class="circlePickersWrap">
              {#each styles.matchStyle as color, key}
                <div class="circlePickerWrap">
                  <label class="circlePickerLabel">#{key + 1}</label>
                  <div class="circlePicker" on:click="{(event) => {changeColorMatch(event, color, key);}}" style="background-color: {color};" >
                  </div>
                  <label class="circlePickerLabel">{color}</label>
                </div>
              {/each}
            </div>
            <h3>Various Other Colors</h3>
            <div id="variousOtherColorsDiv">
            <div id="themeName">
              <label style="margin-top: 7px;">Editor Theme: </label>
              <select class="prefSelector" style="background-color: {styles.editorBackground}; color: {styles.textcolor};" bind:value={styles.editorTheme} on:change="{(event) => {editorThemeChange()}}">
                {#each getEditorThemeList() as item}
                  <option value={item}>{item}</option>
                {/each}
              </select>
            </div>
            <div class="prefColorDiv">
              <label class="styleLabel">Editor Background</label><br>
              <div class="prefColorHolder">
                <div class="circlePicker" on:click="{(event) =>{editEdBk();}}" style="background-color: {styles.editorBackground}; border-color: black; border-width: 2px; border-style: solid;" >
                </div>
                <label class="circlePickerLabel">{styles.editorBackground}</label>
              </div>
            </div>
            <div class="prefColorDiv">
              <label class="styleLabel">Text Color</label><br>
              <div class="prefColorHolder">
                <div class="circlePicker" on:click="{(event) =>{editTextColor();}}" style="background-color: {styles.textcolor};" >
                </div>
                <label class="circlePickerLabel">{styles.textcolor}</label>
              </div>
            </div>
            <div class="prefColorDiv">
              <label class="styleLabel">Application Background</label><br>
              <div class="prefColorHolder">
                <div class="circlePicker" on:click="{(event) =>{editAppBk();}}" style="background-color: {styles.appBackground};" >
                </div>
                <label class="circlePickerLabel">{styles.appBackground}</label>
              </div>
            </div>
            <div class="prefColorDiv">
              <label class="styleLabel">Shadow Color</label><br>
              <div class="prefColorHolder">
                <div class="circlePicker" on:click="{(event) =>{editShadowColor();}}" style="background-color: {styles.shadowColor};" >
                </div>
                <label class="circlePickerLabel">{styles.shadowColor}</label>
              </div>
            </div>
            <div class="prefColorDiv">
              <label class="styleLabel">Script Menu Background</label><br>
              <div class="prefColorHolder">
                <div class="circlePicker" on:click="{(event) =>{editScriptMenuBK();}}" style="background-color: {styles.scriptMenuBackground};" >
                </div>
                <label class="circlePickerLabel">{styles.scriptMenuBackground}</label>
              </div>
            </div>
            <div class="prefColorDiv">
              <label class="styleLabel">Script Menu Text Color</label><br>
              <div class="prefColorHolder">
                <div class="circlePicker" on:click="{(event) =>{editScriptMenuText();}}" style="background-color: {styles.scriptMenuTextColor};" >
                </div>
                <label class="circlePickerLabel">{styles.scriptMenuTextColor}</label>
              </div>
            </div>
            <div class="prefColorDiv">
              <label class="styleLabel">Selection Background Color</label><br>
              <div class="prefColorHolder">
                <div class="circlePicker" on:click="{(event) =>{editSelBkColor();}}" style="background-color: {styles.selectionBackgroundColor};" >
                </div>
                <label class="circlePickerLabel">{styles.selectionBackgroundColor}</label>
              </div>
            </div>                       
            <div class="prefColorDiv">
              <label class="styleLabel">Selection Text Color</label><br>
              <div class="prefColorHolder">
                <div class="circlePicker" on:click="{(event) =>{editSelTextColor();}}" style="background-color: {styles.selectionColor};" >
                </div>
                <label class="circlePickerLabel">{styles.selectionColor}</label>
              </div>
            </div>                      
            <div class="prefColorDiv">
              <label class="styleLabel">Selection Menu Color</label><br>
              <div class="prefColorHolder">
                <div class="circlePicker" on:click="{(event) =>{editSelMenuColor();}}" style="background-color: {styles.selectionMenuColor};" >
                </div>
                <label class="circlePickerLabel">{styles.selectionMenuColor}</label>
              </div>
            </div>                      
            <div class="prefColorDiv">
              <label class="styleLabel">Selection Menu Background Color</label><br>
              <div class="prefColorHolder">
                <div class="circlePicker" on:click="{(event) =>{editSelMenuBKC();}}" style="background-color: {styles.selectionMenuBackgroundColor};" >
                </div>
                <label class="circlePickerLabel">{styles.selectionMenuBackgroundColor}</label>
              </div>
            </div>
            <div class="prefColorDiv">
              <label class="styleLabel">System Script Menu Item</label><br>
              <div class="prefColorHolder">
                <div class="circlePicker" on:click="{(event) =>{editScriptMenuItemSystemColor();}}" style="background-color: {styles.scriptMenuItemSystem};" >
                </div>
                <label class="circlePickerLabel">{styles.scriptMenuItemSystem}</label>
              </div>
            </div>
            <div class="prefColorDiv">
              <label class="styleLabel">User Script Menu Item</label><br>
              <div class="prefColorHolder">
                <div class="circlePicker" on:click="{(event) =>{editScriptMenuItemUserColor();}}" style="background-color: {styles.scriptMenuItemUser};" >
                </div>
                <label class="circlePickerLabel">{styles.scriptMenuItemUser}</label>
              </div>
            </div>
            </div>
            <ColorPicker item="{pickerType}" textColor={styles.textcolor} background={styles.editorBackground} explainText={explanation} color={colorchange} id={colorID} show={showPicker} on:colorChanged="{(event) => {setColorOfButton(event.detail.data.id, event.detail.data.color)}}" on:quitColorPicker="{(event) => { showPicker = false;}}" />
          {:else if pref.name === 'Todos'}
            <PrefTodo preferences="{preferences}" styles="{styles}" on:changeTodos="{(e) => { changeTodos(e.detail.data);}}" />
          {:else if pref.name === 'Node-Red'}
            <PrefNodeRed preferences="{preferences}" styles="{styles}" on:changeNodeRed="{(e) => { changeNodeRed(e.detail.data);}}" />
          {/if}
        </div>
      {/if}
    {/each}
  </div>
  <div id="buttonPanel">
    <button style="background-color: {styles.editorBackground}; color: {styles.textcolor};" on:click="{(event) => { savePreferences();}}">Save Styles & Preference</button>
    <button style="background-color: {styles.editorBackground}; color: {styles.textcolor};" on:click="{(event) => { fire('quitPreferences',{}); keepNewInput = false; }}">Quit Preferences</button>
  </div>
</div>

<style>
  #PreferencePanel {
    display: flex;
    flex-direction: column;
    height: 385px;
    resize: none;
    font-size: 15px;
    overflow-y: scroll;
    overflow-x: hidden;
    list-style: none;
    position: absolute;
    margin: 0px;
    left: 5px;
    z-index: 110;
    -webkit-transition: 0.75s ease-in-out;
    -moz-transition: 0.75s ease-in-out;
    -o-transition: 0.75s ease-in-out;
    transition: 0.75s ease-in-out;    
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    -o-user-select: none;
    user-select: none;
    -webkit-tap-highlight-color: transparent;
    outline-style: none;
  }

  #tabs {
    display: flex;
    flex-direction: row;
    margin: 5px 0px 5px 0px;
    padding: 0px;
    list-style-type: none;
    width: 460px;
    min-width: 460px;
    max-width: 460px;
    padding: 5px 0px 0px 10px;
  }

  #prefListWrap {
    overflow-y: scroll;
    overflow-x: hidden;
    height: 304px;
    padding: 10px 0px 0px 10px;
  }

  #themeName {
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  #buttonPanel {
    flex-direction: row;
    align-content: center;
    padding-top: 10px;
    padding-left: 12px;
    width: 460px;
    min-width: 460px;
    max-width: 460px;
  }

  #buttonPanel button {
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

  .inputRow {
    display: flex;
    flex-direction: row;
    height: 40px;
    width: 100%;
    margin: 5px auto;
  }

  .buttonRow {
    display: flex;
    flex-direction: row;
    align-content: center;
    height: 40px;
    margin: 5px auto;
  }

  .buttonStyle {
    border-radius: 5px;
    border-color: black;
    border-width: 3px;
    font-size: 15px;
    height: 30px;
    text-shadow: 2px 2px 2px black;
    box-shadow: 2px 2px 5px 2px black;
    outline: none;
    margin: 5px 10px;
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

  #buttonPanel button:active {
    box-shadow: inset 2px 2px 5px 2px black;
  }

  #variousOtherColorsDiv {
    margin-left: 10px;
  }

  .PrefCheck {
    margin: 6px 0px 0px 10px;
    font-size: 20px;
  }

  .labelRadialHolder {
    display: flex;
    flex-direction: row;
  }

  .prefInput {
    font-size: 15px;
    border-radius: 5px;
    box-shadow: inset 0px 0px 5px 2px #0f0a16;
    border: 2px #0f0a16;
    min-height: 20px;
    padding: 10px 10px 10px 10px;
    margin: 5px 5px 0px 5px;
    outline: none;
    -webkit-user-select: text;
    -moz-user-select: text;
    -ms-user-select: text;
    -o-user-select: text;
    user-select: text;
  }

  .prefSelector {
    font-size: 15px;
    border-radius: 5px;
    box-shadow: inset 0px 0px 5px 2px #0f0a16;
    border: 2px #0f0a16;
    min-height: 20px;
    padding: 10px 10px 10px 10px;
    margin: 5px 5px 0px 5px;
    outline: none;
    -webkit-user-select: text;
    -moz-user-select: text;
    -ms-user-select: text;
    -o-user-select: text;
    user-select: text;
  }

  .prefColorDiv {
    display: flex;
    flex-direction: column;
    margin: 20px 0px 0px 0px;
  }

  .prefColorHolder {
    display: flex;
    flex-flow: row;
  }   
   
  .circlePickersWrap {
    display: flex;
    flex-flow: column wrap;
    padding: 0px 0px 0px 10px;
    height: 165px;
  }

  .circlePickerWrap {
    display: flex;
    flex-direction: row;
    width: 200px;
  }

  .circlePickerLabel {
    font-size: 25px;
    margin: 0px 10px 0px 10px;
    user-select: text;
  }

  .styleLabel {
    font-size: 18px;
    margin: 0px 0px 0px 0px;
    user-select: none;
  }

  .circlePicker {
    height: 30px;
    min-height: 30px;
    width: 30px;
    min-width: 30px;
    border-radius: 30px;
    cursor: pointer;
  }

  .current {
    color: white;
  }

  .notCurrent {
    color: gray;
  }    

  .tabName {
    font-size: 10px;
    list-style: none;
    margin: 0px 10px 0px 0px;
    padding: 0px;
    cursor: pointer;
  }

  .visible {
    width: 467px;
    top: 05px;
    box-shadow: inset 0px 0px 5px 2px black;
    padding: 7px;
  }

  .invisible {
    width: 0px;
    top: 05px;
    padding: 0px;
    box-shadow: inset 0px 0px 0px 0px black;
  }

  .prefListPanel {
    display: flex;
    flex-direction: column;
    width: 460px;
    min-width: 460px;
    max-width: 460px;
  }
</style>

<script>
  import ColorPicker from './ColorPicker.svelte';
  import PrefTodo from './PrefTodo.svelte';
  import PrefNodeRed from './PrefNodeRed.svelte';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();
  
  export let openPanel = false;
  export let styles = {};
  export let preferences = {};
  export let currentID = 0;
  export let ScriptPad = {};

  let colorchange = "";
  let colorID = 0;
  let showPicker = false;
  let currentPref = 'General';
  let matchColor = false;
  let buttonColor = false;
  let keepNewInput = false;
  let pickerType = '';
  let explanation;
  let prefs = [{
  name: "General"
  },{
    name: "Theme"
  }, {
    name: "Todos"
  }, {
    name: "Node-Red"
  }];

  $: setKeepInput(preferences.style, keepNewInput);
  $: explanation = (Number.isInteger(colorID)) ? "#" + (colorID+1) + " of " + pickerType : pickerType + " Color";

  function fire(name, data) {
    dispatch(name, {
      data: data
    });
  }

  function changeTodos(todos) {
    preferences.taskpaper = todos.taskpaper;
    preferences.UseTaskPaper = todos.UseTaskPaper;
    preferences.UseNotePlan = todos.UseNotePlan;
    savePreferences();
  }

  function changeNodeRed(nodered) {
    preferences.REDAuto = nodered.auto;
    preferences.REDDashboard = nodered.dashboard;
    savePreferences();
  }

  function savePreferences() {
    fire('changeStyles', styles); 
    fire('changePreferences', preferences);
    keepNewInput = false;
  }

  function getEditorThemeList() {
    return ScriptPad.getEditorThemes();
  }

  function editEdBk() {
     colorchange = styles.editorBackground;
     colorID = 'edbk';
     pickerType = 'Editor Background';
     showPicker = true; 
  }

  function editTextColor() {
    colorchange = styles.textcolor;
    colorID = 'edtc';
    pickerType = 'Text Color';
    showPicker = true;
  }

  function editAppBk() {
    colorchange = styles.appBackground;
    colorID = 'edab';
    pickerType = 'Application Background Color';
    showPicker = true;
  }

  function editShadowColor() {
    colorchange = styles.shadowColor;
    colorID = 'edsc';
    pickerType = 'Shadow Color';
    showPicker = true;
  }

  function editScriptMenuBK() {
    colorchange = styles.scriptMenuBackground;
    colorID = 'edsmbk';
    pickerType = 'Script Menu Background Color';
    showPicker = true;
  }

  function editScriptMenuText() {
    colorchange = styles.scriptMenuTextColor;
    colorID = 'edsmtc';
    pickerType = 'Script Menu Text Color';
    showPicker = true;
  }

  function editSelBkColor() {
    colorchange = styles.selectionBackgroundColor;
    colorID = 'edsbk';
    pickerType = 'Selection Background Color';
    showPicker = true;
  }

  function editSelTextColor() {
    colorchange = styles.selectionColor;
    colorID = 'edstc';
    pickerType = 'Selection Text Color';
    showPicker = true;
  }

  function editSelMenuColor() {
    colorchange = styles.selectionMenuColor;
    colorID = 'edsmc';
    pickerType = 'Selection Menu Color';
    showPicker = true;
  }

  function editSelMenuBKC() {
    colorchange = styles.selectionMenuBackgroundColor;
    colorID = 'edsmbc';
    pickerType = 'Selection Menu Background Color';
    showPicker = true;
  }

  function editScriptMenuItemUserColor() {
    colorchange = styles.scriptMenuItemUser;
    colorID = 'edsmuc';
    pickerType = 'Script Menu User Color';
    showPicker = true;
  }

  function editScriptMenuItemSystemColor() {
    colorchange = styles.scriptMenuItemSystem;
    colorID = 'edsmsc';
    pickerType = 'Script Menu  Color';
    showPicker = true;
  }

  function editorThemeChange() {
    console.log("Editor Theme Change.")
  }
  
  function setKeepInput(style, test) {
    if((style === 'New')&&(!test)) {
      keepNewInput = true;
    }
  }

  function styleSelectorChange() {
    if(!keepNewInput) {
      getUserStyles(preferences.style);
    }
  }

  function getStyleList() {
    return ScriptPad.getStyleNames();
  }

  function setColorOfButton(id, color) {
    if(buttonColor) {
      styles.buttons[id].color = color;
    } else if(matchColor) {
      styles.matchStyle[id] = color;
    } else {
      switch(id) {
        case 'edbk':
          styles.editorBackground = color;
          break;
        case 'edtc':
          styles.textcolor = color;
          break;
        case 'edab':
          styles.appBackground = color;
          break;
        case 'edsc':
          styles.shadowColor = color;
          break;
        case 'edsmtc':
          styles.scriptMenuTextColor = color;
          break;
        case 'edsbk':
          styles.selectionBackgroundColor = color;
          break;
        case 'edsmbk':
          styles.scriptMenuBackground = color;
          break;
        case 'edstc':
          styles.selectionColor = color;
          break;
        case 'edsmc':
          styles.selectionMenuColor = color;
          break;
        case 'edsmbc':
          styles.selectionMenuBackgroundColor = color;
          break;
        case 'edsmuc':
          styles.scriptMenuItemUser = color;
          break;
        case 'edsmsc':
          styles.scriptMenuItemSystem = color;
          break;
        default:
          console.log("Invalid id.");
          break;
      }
    }
    showPicker = false;
    buttonColor = false;
    matchColor = false;
  }

  function changeColorMatch(event, color, key) {
    colorchange = color;
    colorID = key;
    showPicker = true;
    matchColor = true;
    buttonColor = false;
    pickerType = 'SubExpression';
  }

  function changeColor(event, button) {
    colorchange = button.color;
    colorID = button.id;
    showPicker = true;
    matchColor = false;
    buttonColor = true;
    pickerType = 'Circle';
  }
  
  function setNewCurrent(newCurrent) {
    currentPref = newCurrent;
  }

  function getUserStyles(styleName) {
    globalThis.style = ScriptPad.getStyleFile(styleName);
    globalThis.style.buttons[currentID].selected = true;
    fire('changeStyles', globalThis.style);
  }
</script>
