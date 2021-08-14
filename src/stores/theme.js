import { writable } from 'svelte/store';

let defaultTheme = {
  font: "Fira Code, Menlo",
  fontSize: "16pt",
  textAreaColor: '#454158',
  backgroundColor: '#22212C',
  textColor: '#80ffea',
  borderColor: '#1B1A23',
  Cyan: "#80FFEA",
  Green: "#8AFF80",
  Orange: "#FFCA80",
  Pink: "#FF80BF",
  Purple: "#9580FF",
  Red: "#FF9580",
  Yellow: "#FFFF80",
  Circle0: '#80FFEA',
  Circle1: '#8AFF80',
  Circle2: '#FFCA80',
  Circle3: '#FF80BF',
  Circle4: '#9580FF',
  Circle5: '#FF9580',
  Circle6: 'blue',
  Circle7: 'green',
  Circle8: 'red',
  Circle9: 'purple',
};

export const theme = writable( defaultTheme );

