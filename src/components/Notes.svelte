<div 
  id='notes' 
  style="background-color: {$theme.backgroundColor}; font-family: {$theme.font}; color: {$theme.textColor}; font-size: {$theme.fontSize};"
>
  <div 
    id="editorRow"
  >
    <CodeMirror height='530px' 
                width='900px' 
                config={editorConfig}
                initFinished={initFinished}
                on:textChange='{(event) => {textChanged(event.detail.data)}}' 
                on:editorChange='{(event) => {editorChange(event.detail.data); }}'
    />
    <div 
      id="noteButtons"
    >
      {#each $theme.buttons as button, key}
        <div 
          class="noteButton {$currentNote === $theme.buttons[key].id ? 'selectedButton' : ''}"
          on:click={() => {
            openNote(key); 
          }}
          style="background-color: {$theme.buttons[key].color};"
        >
        </div>
      {/each}
    </div>
  </div>
  <div 
    id='buttonRow' 
  >
    <button
      on:click={viewEmailIt}
      style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
    >
      EmailIt
    </button>
    <button
      on:click={viewLogs}
      style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
    >
      Logs
    </button>
    <button
      on:click={viewScriptsMenu}
      style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
    >
      Scripts
    </button>
    <button
      on:click={viewTemplateMenu}
      style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
    >
      Templates
    </button>
    <button
      on:click={viewScripts}
      style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
    >
      Edit Scripts
    </button>
    <button
      on:click={viewTemplates}
      style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
    >
      Edit Templates
    </button>
  </div>
</div>

<style> 
  #notes {
    display: flex;
    flex-direction: column;
    padding: 10px;
    height: 100%;
    width: 100%;
  }
  
  #buttonRow {
    display: flex;
    flex-direction: row;
    margin: 10px auto;
    position: absolute;
    bottom: 0px;
    width: 100%;
    height: 40px;
  }
  
#buttonRow button {
    border-radius: 10px;
    padding: 5px 10px 5px 10px;
    margin: 0px 5px;
    max-height: 40px;
    height: 40px;
    cursor: pointer;
  }

  #editorRow {
    display: flex;
    flex-direction: row;
    margin: 0px;
    padding: 0px;
  }

  #noteButtons {
    display: flex;
    flex-direction: column;
    width: 80px;
    height: 530px;
    position: absolute;
    right: 5px;
  }

  .noteButton {
    height: 45px;
    width: 45px;
    margin: auto;
    padding: 0px;
    border-radius: 50px;
    border: solid 2px transparent;
    cursor: pointer;
  }

  .selectedButton {
    box-shadow: inset 0px 0px 20px 10px rgba(0,0,0,0.6);
  }
</style> 

<script> 
  import { onMount, tick } from 'svelte';
  import { fetch, Body } from "@tauri-apps/api/http";
  import CodeMirror from '../components/CodeMirror.svelte';
  import { state } from '../stores/state.js';
  import { theme } from '../stores/theme.js';
  import { currentNote } from '../stores/currentNote.js';
  import { storedText } from '../stores/storedText.js';
  import { storedCursor } from '../stores/storedCursor.js';
  import { noteEditor } from '../stores/noteEditor.js';
  import { showScripts } from '../stores/showScripts.js';
  import { showTemplates } from '../stores/showTemplates.js';
  
  let editorConfig = {
    language: 'markdown',
    lineNumbers: true,
    lineWrapping: true,
    lineHighlight: true
  };
  let initFinished = false;

  onMount(() => {
    // 
    // Load everything for working with the notes:
    //
    loadNotes();
  });

  function loadNotes() {
    getNote(0);
    getNote(1);
    getNote(2);
    getNote(3);
    getNote(4);
    getNote(5);
    getNote(6);
    getNote(7);
    getNote(8);
    getNote(9, async () => {
      //
      // When last note is loaded, setup for displaying the 
      // proper note.
      //
      initFinished = true;
      await tick();
      openNote($currentNote);
      focus();
    });
  }

  function getNote(id, callback) {
    fetch(`http://localhost:9978/api/note/${id}/w`, {
        method: 'GET',
        headers: {
          'Content-type': 'application/json'
        }
      }).then(resp => {
        return resp.data;
      })
      .then(data => {
        $storedText[id] = data.note;
        if(typeof callback !== 'undefined') callback();
      });
  }

  function saveNote(id) {
    var text = $storedText[id];
    fetch(`http://localhost:9978/api/note/${id}/w`, {
        method: 'PUT',
        headers: {
          'Content-type': 'application/json'
        },
        body: Body.json({
          note: text
        })
      })
  }

  function editorChange(e) {
    $noteEditor = e;
  }

  function textChanged(textCursor) {
    $storedText[$currentNote] = textCursor.value;
    $storedCursor[$currentNote] = textCursor.cursor;
    saveNote($currentNote);
  }

  function focus() {
    if($noteEditor !== null) {
      $noteEditor.focus();
    }
  }

  function viewLogs() {
    storeCurrentCursor();
    $state = 'viewlog';
  }

  function viewEmailIt() {
    storeCurrentCursor();
    $state = 'emailit';
  }

  function storeCurrentCursor() {
      $storedCursor[$currentNote] = $noteEditor.getCursor();
  }

  function openNote(id) {
    $currentNote = id;
    $noteEditor.setValue($storedText[$currentNote]);
    var cur = parseInt($storedCursor[$currentNote]);
    if(!Number.isInteger(cur)) cur = 0;
    $noteEditor.setCursor(cur);
    focus();
  }

  function viewScriptsMenu() {
    $showScripts = ! $showScripts;
  }

  function viewTemplateMenu() {
    $showTemplates = ! $showTemplates;
  }

  function viewTemplates() {
    $state = 'templates';
  }

  function viewScripts() {
    $state = 'scripts';
  }
</script> 

