import type { User } from '$lib/types/User';
import { writable } from 'svelte/store';

export const activeUsersStore = writable<User[]>([]);
