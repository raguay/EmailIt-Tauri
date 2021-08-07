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
  import EmailIt from './components/EmailIt.svelte';
  import ViewLog from './components/ViewLog.svelte';
  import Notes from './components/Notes.svelte';
  import { state } from './stores/state.js';

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

