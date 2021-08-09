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

<script>
  import { onMount, afterUpdate } from 'svelte';
  import { getMatches } from "@tauri-apps/api/cli";
  import { appWindow } from "@tauri-apps/api/window";
  import EmailIt from './components/EmailIt.svelte';
  import ViewLog from './components/ViewLog.svelte';
  import Notes from './components/Notes.svelte';
  import { state } from './stores/state.js';
  import { commandLineEmail } from './stores/commandLineEmail.js';

  let starting = true;

  onMount(() => {
    getMatches().then((matches) => {
      if((typeof matches.args.emails.value !== 'undefined')&&(matches.args.emails.value)) {
        $commandLineEmail = matches.args.emails.value;
      }
    });
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
      }
    }
  }
</script>

