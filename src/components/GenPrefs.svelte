<div>
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
</div>

