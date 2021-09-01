<svelte:window 
  on:keydown={keyDownProcessor}
/>

{#if $state === 'emailit'}
  <EmailIt />
{:else if $state === 'viewlog'}
  <ViewLog />
{:else if $state === 'notes'}
  <Notes />
{:else if $state === 'scripts'}
  <ScriptsEditor />
{:else if $state === 'templates'}
  <TemplatesEditor />
{:else if $state === 'preferences'}
  <Preferences />
{/if}
<ScriptMenu 
/>
<TemplateMenu
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
  import TemplateMenu from './components/TemplateMenu.svelte';
  import ScriptsEditor from './components/ScriptsEditor.svelte';
  import TemplatesEditor from './components/TemplatesEditor.svelte';
  import Preferences from './components/Preferences.svelte';
  import { state } from './stores/state.js';
  import { scripts } from './stores/scripts.js';
  import { showScripts } from './stores/showScripts.js';
  import { templates } from './stores/templates.js';
  import { showTemplates } from './stores/showTemplates.js';
  import { commandLineEmail } from './stores/commandLineEmail.js';

  let starting = true;

  onMount(() => {
    getMatches().then((matches) => {
      if((typeof matches.args.emails.value !== 'undefined')&&(matches.args.emails.value)) {
        $commandLineEmail = matches.args.emails.value;
      }
    });
    getScriptsList();
    getTemplatesList();
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

  function getTemplatesList() {
    fetch('http://localhost:9978/api/template/list', {
        method: 'GET',
        headers: {
          'Content-type': 'application/json'
        }
      }).then(resp => {
        return resp.data;
      })
      .then(data => {
        $templates = data.templates;
        if(typeof callback !== 'undefined') callback();
      });
  }

  function keyDownProcessor(e) {
    if(e.metaKey && (e.key === ',')) {
      e.preventDefault();
      $state = 'preferences';
    } else if(e.ctrlKey) {
      switch(e.key) {
        case 'e': 
          $state = 'emailit';
          e.preventDefault();
        break;

        case 'v':
          $state = 'viewlog';
          e.preventDefault();
        break;

        case 'n':
          $state = 'notes';
          e.preventDefault();
        break;

        case 'm':
          $showScripts = !$showScripts;
          e.preventDefault();
        break;

        case 't':
          $showTemplates = ! $showTemplates;
          e.preventDefault();
        break;

        case 'p':
          $state = 'preferences';
          e.preventDefault();
      }
    }
  }
</script>

