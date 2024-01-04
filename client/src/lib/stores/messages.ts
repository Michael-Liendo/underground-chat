import { writable } from 'svelte/store';

export interface Message {
  content: string;
  userName: string;
}

export const messagesStore = writable<Message[]>([]);
