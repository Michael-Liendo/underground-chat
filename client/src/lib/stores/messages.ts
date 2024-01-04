import { writable } from 'svelte/store';

export interface Message {
  content: string;
  username: string;
  room: string;
}

export const messagesStore = writable<Message[]>([]);
