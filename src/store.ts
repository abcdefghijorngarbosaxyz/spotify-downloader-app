import { writable } from 'svelte/store';
import { useLocalStorage } from './lib/utils/useLocalStorage';
import { LocalKeys } from './constants';

export const DOWNLOAD_FOLDER = writable<string>(
  useLocalStorage('get', LocalKeys.DOWNLOAD_FOLDER) ?? ''
);
