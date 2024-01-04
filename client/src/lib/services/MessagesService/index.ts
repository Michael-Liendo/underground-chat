import { messagesStore, type Message } from '$lib/stores/messages';
import { io } from 'socket.io-client';
import UserService from '../UserService';

export const socket = io(`${import.meta.env.VITE_API_URL}/ws`, {
  auth: { username: await UserService.get() },
});

socket.on('chat message', (message: Message) => {
  messagesStore.update((messages) => [
    ...messages,
    message,
  ]);
});

export default class MessagesService {
  static async send(content: string, room: string): Promise<string | null> {
    try {
      const username = UserService.get()
      socket.emit('chat message', { content, room, username });

      messagesStore.update((messages) => [
        ...messages,
        { room: room, content: content, username: `${username} (You)` },
      ]);

      return 'send';
    } catch (error) {
      console.error('Error send message:', error);
      return null;
    }
  }
}
