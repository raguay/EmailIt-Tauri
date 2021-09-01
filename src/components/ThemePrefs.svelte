<div id="themeName">
  <label>Name of theme: </label>
  {#if (style === 'New')||(keepNewInput)}
    <input 
      class="prefInput" 
      style="background-color: {$theme.textAreaColor}; color: {$theme.textColor};"
      bind:value={style}
    >
  {:else}
    <select
      class="prefSelector" 
      style="background-color: {$theme.textAreaColor}; color: {$theme.textColor};"
      bind:value={style}
      on:change="{(event) => {styleSelectorChange()}}"
    >
    {#each getStyleList() as item}
      <option value={item}>{item}</option>
    {/each}
    </select>
  {/if}
</div>
<h3>Circle Colors</h3>
<div class="circlePickersWrap">
  {#each $theme.buttons as button, key}
    <div class="circlePickerWrap">
      <label class="circlePickerLabel">#{key}</label>
      <div class="circlePicker" on:click="{(event) => {changeColor(button);}}" style="background-color: {button.color};" >
      </div>
      <label class="circlePickerLabel">{button.color}</label>
    </div>
  {/each}
</div>
<h3>Various Other Colors</h3>
<div id="variousOtherColorsDiv">
  <div class="circlePickerWrap">
    <label class="circlePickerLabel">
      textAreaColor
    </label>
    <div class="circlePicker" on:click="{(event) => {changeColor({
      id: 'textAreaColor',
      color: $theme.textAreaColor
    });}}" style="background-color: {$theme.textAreaColor};" >
    </div>
    <label class="circlePickerLabel">{$theme.textAreaColor}</label>
  </div>
  <div class="circlePickerWrap">
    <label class="circlePickerLabel">
      backgroundColor
    </label>
    <div class="circlePicker" on:click="{(event) => {changeColor({
      id: 'backgroundColor',
      color: $theme.backgroundColor
    });}}" style="background-color: {$theme.backgroundColor};" >
    </div>
    <label class="circlePickerLabel">{$theme.backgroundColor}</label>
  </div>
  <div class="circlePickerWrap">
    <label class="circlePickerLabel">
      textColor
    </label>
    <div class="circlePicker" on:click="{(event) => {changeColor({
      id: 'textColor',
      color: $theme.textColor
    });}}" style="background-color: {$theme.textColor};" >
    </div>
    <label class="circlePickerLabel">{$theme.textColor}</label>
  </div>
  <div class="circlePickerWrap">
    <label class="circlePickerLabel">
      borderColor
    </label>
    <div class="circlePicker" on:click="{(event) => {changeColor({
      id: 'borderColor',
      color: $theme.borderColor
    });}}" style="background-color: {$theme.borderColor};" >
    </div>
    <label class="circlePickerLabel">{$theme.borderColor}</label>
  </div>
  <div class="circlePickerWrap">
    <label class="circlePickerLabel">
      functionColor
    </label>
    <div class="circlePicker" on:click="{(event) => {changeColor({
      id: 'functionColor',
      color: $theme.functionColor
    });}}" style="background-color: {$theme.functionColor};" >
    </div>
    <label class="circlePickerLabel">{$theme.functionColor}</label>
  </div>
  <div class="circlePickerWrap">
    <label class="circlePickerLabel">
      stringColor
    </label>
    <div class="circlePicker" on:click="{(event) => {changeColor({
      id: 'stringColor',
      color: $theme.stringColor
    });}}" style="background-color: {$theme.stringColor};" >
    </div>
    <label class="circlePickerLabel">{$theme.stringColor}</label>
  </div>
  <div class="circlePickerWrap">
    <label class="circlePickerLabel">
      constantColor
    </label>
    <div class="circlePicker" on:click="{(event) => {changeColor({
      id: 'constantColor',
      color: $theme.constantColor
    });}}" style="background-color: {$theme.constantColor};" >
    </div>
    <label class="circlePickerLabel">{$theme.constantColor}</label>
  </div>
  <div class="circlePickerWrap">
    <label class="circlePickerLabel">
      keywordColor
    </label>
    <div class="circlePicker" on:click="{(event) => {changeColor({
      id: 'keywordColor',
      color: $theme.keywordColor
    });}}" style="background-color: {$theme.keywordColor};" >
    </div>
    <label class="circlePickerLabel">{$theme.keywordColor}</label>
  </div>
  <div class="circlePickerWrap">
    <label class="circlePickerLabel">
      highlightBackgroundColor
    </label>
    <div class="circlePicker" on:click="{(event) => {changeColor({
      id: 'highlightBackgroundColor',
      color: $theme.highlightBackgroundColor
    });}}" style="background-color: {$theme.highlightBackgroundColor};" >
    </div>
    <label class="circlePickerLabel">{$theme.highlightBackgroundColor}</label>
  </div>
  <div class="circlePickerWrap">
    <label class="circlePickerLabel">
      selectionColor
    </label>
    <div class="circlePicker" on:click="{(event) => {changeColor({
      id: 'selectionColor',
      color: $theme.selectionColor
    });}}" style="background-color: {$theme.selectionColor};" >
    </div>
    <label class="circlePickerLabel">{$theme.selectionColor}</label>
  </div>
  <div class="circlePickerWrap">
    <label class="circlePickerLabel">
      Cyan
    </label>
    <div class="circlePicker" on:click="{(event) => {changeColor({
      id: 'Cyan',
      color: $theme.Cyan
    });}}" style="background-color: {$theme.Cyan};" >
    </div>
    <label class="circlePickerLabel">{$theme.Cyan}</label>
  </div>
  <div class="circlePickerWrap">
    <label class="circlePickerLabel">
      Green
    </label>
    <div class="circlePicker" on:click="{(event) => {changeColor({
      id: 'Green',
      color: $theme.Green
    });}}" style="background-color: {$theme.Green};" >
    </div>
    <label class="circlePickerLabel">{$theme.Green}</label>
  </div>
  <div class="circlePickerWrap">
    <label class="circlePickerLabel">
      Orange
    </label>
    <div class="circlePicker" on:click="{(event) => {changeColor({
      id: 'Orange',
      color: $theme.Orange
    });}}" style="background-color: {$theme.Orange};" >
    </div>
    <label class="circlePickerLabel">{$theme.Orange}</label>
  </div>
  <div class="circlePickerWrap">
    <label class="circlePickerLabel">
      Pink
    </label>
    <div class="circlePicker" on:click="{(event) => {changeColor({
      id: 'Pink',
      color: $theme.Pink
    });}}" style="background-color: {$theme.Pink};" >
    </div>
    <label class="circlePickerLabel">{$theme.Pink}</label>
  </div>
  <div class="circlePickerWrap">
    <label class="circlePickerLabel">
      Purple
    </label>
    <div class="circlePicker" on:click="{(event) => {changeColor({
      id: 'Purple',
      color: $theme.Purple
    });}}" style="background-color: {$theme.Purple};" >
    </div>
    <label class="circlePickerLabel">{$theme.Purple}</label>
  </div>
  <div class="circlePickerWrap">
    <label class="circlePickerLabel">
      Red
    </label>
    <div class="circlePicker" on:click="{(event) => {changeColor({
      id: 'Red',
      color: $theme.Red
    });}}" style="background-color: {$theme.Red};" >
    </div>
    <label class="circlePickerLabel">{$theme.Red}</label>
  </div>
  <div class="circlePickerWrap">
    <label class="circlePickerLabel">
      Yellow
    </label>
    <div class="circlePicker" on:click="{(event) => {changeColor({
      id: 'Yellow',
      color: $theme.Yellow
    });}}" style="background-color: {$theme.Yellow};" >
    </div>
    <label class="circlePickerLabel">{$theme.Yellow}</label>
  </div>
</div>
{#if $theme !== undefined}
  <ColorPicker 
    item="{pickerType}" 
    explainText={explanation}
    color={colorchange}
    id={colorID}
    show={showPicker}
    on:colorChanged="{(event) => {setColor(event.detail.data.id, event.detail.data.color)}}"
    on:quitColorPicker="{(event) => { showPicker = false;}}"
  />
{/if}

<style>
  #themeName {
    display: flex;
    flex-direction: row;
    align-items: center;
  }
  
  #variousOtherColorsDiv {
    margin-left: 10px;
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
    margin: 10px auto;
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
    border: solid 1px white;
  }
</style>

<script>
  import ColorPicker from './ColorPicker.svelte';
  import { theme } from '../stores/theme.js';
  
  let colorchange = "";
  let colorID = 0;
  let showPicker = false;
  let buttonColor = false;
  let keepNewInput = false;
  let pickerType = '';
  let style = 'default';
  let explanation;
  
  function setKeepInput(style, test) {
    if((style === 'New')&&(!test)) {
      keepNewInput = true;
    }
  }

  function styleSelectorChange() {
    if(!keepNewInput) {
    }
  }

  function getStyleList() {
    var result = ['default','New'];

    return(result);
  }

  function setColor(id, color) {
    if(buttonColor) {
      $theme.buttons[id].color = color;
    } else {
      switch(id) {
        case 'textAreaColor':
          $theme.textAreaColor = color;
        break;
        
        case 'backgroundColor':
          $theme.backgroundColor = color;
        break;
        
        case 'textColor':
          $theme.textColor = color;
        break;
        
        case 'borderColor':
          $theme.borderColor = color;
        break;
        
        case 'Cyan':
          $theme.Cyan = color;
        break;
        
        case 'Green':
          $theme.Green = color;
        break;
        
        case 'Orange':
          $theme.Orange = color;
        break;
        
        case 'Pink':
          $theme.Pink = color;
        break;
        
        case 'Purple':
          $theme.Purple = color;
        break;
        
        case 'Red':
          $theme.Red = color;
        break;
        
        case 'Yellow':
          $theme.Yellow = color;
        break;
        
        case 'functionColor':
          $theme.functionColor = color;
        break;
        
        case 'stringColor':
          $theme.stringColor = color;
        break;
        
        case 'constantColor':
          $theme.constantColor = color;
        break;
        
        case 'keywordColor':
          $theme.keywordColor = color;
        break;
        
        case 'highlightBackgroundColor':
          $theme.highlightBackgroundColor = color;
        break;
        
        case 'selectionColor':
          $theme.selectionColor = color;
        break;
        
        default:
          console.log("Invalid id.");
          break;
      }
    }
    showPicker = false;
    buttonColor = false;
  }

  function changeColor(button) {
    colorchange = button.color;
    colorID = button.id;
    showPicker = true;
    if(isNaN(colorID)) {
      buttonColor = false;
      pickerType = colorID;
      explanation = `${colorID}`;
    } else {
      buttonColor = true;
      pickerType = 'Circle';
      explanation = `#${colorID} Circle`;
    }
  }
</script>
