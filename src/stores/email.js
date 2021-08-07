import { writable } from 'svelte/store';

export const email = writable( {
  to: undefined,
  subject: undefined,
  body: undefined
} );

