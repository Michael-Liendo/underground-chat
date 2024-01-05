import { messagesStore } from '$lib/stores/messages';
import { io } from 'socket.io-client';
import UserService from '../UserService';

import type { Message } from '$lib/types/Messages';
import type { Join } from '$lib/types/Join';
import { activeUsersStore } from '$lib/stores/users';

export const socket = io(`${import.meta.env.VITE_API_URL}/ws`, {
  auth: { username: await UserService.get() },
});

socket.on('chat message', (message: Message) => {
  messagesStore.update((messages) => [...messages, message]);
});

socket.on('join', (join:Join) =>{
  activeUsersStore.update((users) => [...users, {username: join.username}]);
})


export default class MessagesService {
  static async send(content: string, room: string): Promise<string | null> {
    try {
      const username = UserService.get();
      const created_at = new Date();
      socket.emit('chat message', { content, room, username, created_at });

      messagesStore.update((messages) => [
        ...messages,
        {
          room: room,
          content: content,
          username: `${username} (You)`,
          created_at,
        },
      ]);

      return 'send';
    } catch (error) {
      console.error('Error send message:', error);
      return null;
    }
  }
}
