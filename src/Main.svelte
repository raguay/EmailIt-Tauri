<svelte:window 
  on:keydown={keyDownProcessor}
/>

{#if $state === 'emailit'}
  <EmailIt />
{:else if $state === 'viewlog'}
  <ViewLog />
{:else if $state === 'notes'}
  <Notes />
{/if}
<ScriptMenu 
/>

<script>
  import { onMount, afterUpdate } from 'svelte';
  import { getMatches } from "@tauri-apps/api/cli";
  import { appWindow } from "@tauri-apps/api/window";
  import { fetch, Body } from "@tauri-apps/api/http";
  import EmailIt from './components/EmailIt.svelte';
  import ViewLog from './components/ViewLog.svelte';
  import Notes from './components/Notes.svelte';
  import ScriptMenu from './components/ScriptMenu.svelte';
  import { state } from './stores/state.js';
  import { scripts } from './stores/scripts.js';
  import { showScripts } from './stores/showScripts.js';
  import { commandLineEmail } from './stores/commandLineEmail.js';

  let starting = true;

  onMount(() => {
    getMatches().then((matches) => {
      if((typeof matches.args.emails.value !== 'undefined')&&(matches.args.emails.value)) {
        $commandLineEmail = matches.args.emails.value;
      }
    });
    getScriptsList();
  });

  afterUpdate(() => {
    if(starting) {
      appWindow.isVisible()
        .then(visible => {
          if(!visible) appWindow.show();
        });
      starting = false;
    }
  });

  function getScriptsList() {
    fetch('http://localhost:9978/api/scripts/list', {
        method: 'GET',
        headers: {
          'Content-type': 'application/json'
        }
      }).then(resp => {
        return resp.data;
      })
      .then(data => {
        $scripts = data.data;
        if(typeof callback !== 'undefined') callback();
      });
  }

  function keyDownProcessor(e) {
    if(e.ctrlKey) {
      switch(e.key) {
        case 'e': 
          state.set('emailit');
          e.preventDefault();
        break;

        case 'v':
          state.set('viewlog');
          e.preventDefault();
        break;

        case 'n':
          state.set('notes');
          e.preventDefault();
        break;

        case 'm':
          $showScripts = !$showScripts;
          e.preventDefault();
        break;
      }
    }
  }
</script>

