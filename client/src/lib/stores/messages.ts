import { writable } from 'svelte/store';

import type { Message } from '$lib/types/Messages';

export const messagesStore = writable<Message[]>([]);
