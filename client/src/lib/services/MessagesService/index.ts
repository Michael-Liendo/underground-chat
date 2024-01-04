import { io } from 'socket.io-client';
import { messagesStore, type Message } from '$lib/stores/messages';

export const socket = io(`${import.meta.env.VITE_API_URL}/ws`);

socket.on('chat message', (message: unknown, name: string) => {
  messagesStore.update((messages) => [
    ...messages,
    { content: message as string, userName: 'any' },
  ]);
});

export default class MessagesService {
  static async send(content: string, chatID: string): Promise<string | null> {
    try {
      socket.emit('chat message', content);

      return 'send';
    } catch (error) {
      console.error('Error send message:', error);
      return null;
    }
  }
}
