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
      <div 
        class='noteButton'
        on:click={() => {
          openNote(0); 
        }}
        style="height: {$currentNote === 0 ? '50px' : '45px'}; width: {$currentNote === 0 ? '50px' : '45px'};"
      >
      </div>
      <div 
        class='noteButton'
        on:click={() => { 
          storeCurrentCursor();
          openNote(1); 
        }}
        style="height: {$currentNote === 1 ? '50px' : '45px'}; width: {$currentNote === 1 ? '50px' : '45px'};"
      >
      </div>
      <div 
        class='noteButton'
        on:click={() => { 
          storeCurrentCursor();
          openNote(2); 
        }}
        style="height: {$currentNote === 2 ? '50px' : '45px'}; width: {$currentNote === 2 ? '50px' : '45px'};"
      >
      </div>
      <div 
        class='noteButton'
        on:click={() => { 
          storeCurrentCursor();
          openNote(3); 
        }}
        style="height: {$currentNote === 3 ? '50px' : '45px'}; width: {$currentNote === 3 ? '50px' : '45px'};"
      >
      </div>
      <div 
        class='noteButton'
        on:click={() => { 
          storeCurrentCursor();
          openNote(4); 
        }}
        style="height: {$currentNote === 4 ? '50px' : '45px'}; width: {$currentNote === 4 ? '50px' : '45px'};"
      >
      </div>
      <div 
        class='noteButton'
        on:click={() => { 
          storeCurrentCursor();
          openNote(5); 
        }}
        style="height: {$currentNote === 5 ? '50px' : '45px'}; width: {$currentNote === 5 ? '50px' : '45px'};"
      >
      </div>
      <div 
        class='noteButton'
        on:click={() => { 
          storeCurrentCursor();
          openNote(6); 
        }}
        style="height: {$currentNote === 6 ? '50px' : '45px'}; width: {$currentNote === 6 ? '50px' : '45px'};"
      >
      </div>
      <div 
        class='noteButton'
        on:click={() => { 
          storeCurrentCursor();
          openNote(7); 
        }}
        style="height: {$currentNote === 7 ? '50px' : '45px'}; width: {$currentNote === 7 ? '50px' : '45px'};"
      >
      </div>
      <div 
        class='noteButton'
        on:click={() => { 
          storeCurrentCursor();
          openNote(8); 
        }}
        style="height: {$currentNote === 8 ? '50px' : '45px'}; width: {$currentNote === 8 ? '50px' : '45px'};"
      >
      </div>
      <div 
        class='noteButton'
        on:click= {() => { 
          storeCurrentCursor();
          openNote(9); 
        }}
        style="height: {$currentNote === 9 ? '50px' : '45px'}; width: {$currentNote === 9 ? '50px' : '45px'};"
      >
      </div>
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
    padding: 5px 20px 5px 20px;
    margin: 0px 20px;
    max-height: 40px;
    height: 40px;
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
    height: 50px;
    width: 50px;
    margin: auto;
    padding: 0px;
    border-radius: 50px;
    border: solid 2px transparent;
    background-color: blue;
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
</script> 

