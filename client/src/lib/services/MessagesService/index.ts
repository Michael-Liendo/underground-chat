import { messagesStore } from '$lib/stores/messages';
import { io } from 'socket.io-client';
import UserService from '../UserService';


export const socket = io(`${import.meta.env.VITE_API_URL}/ws`, {
  auth: { username: await UserService.get() },
});

socket.on('chat message', (message: string, name: string) => {
  messagesStore.update((messages) => [
    ...messages,
    { content: message, userName: name },
  ]);
});

export default class MessagesService {
  static async send(content: string, chatID: string): Promise<string | null> {
    try {
      socket.emit('chat message', content, UserService.get(), chatID);

      messagesStore.update((messages) => [
        ...messages,
        { content: content as string, userName: `${UserService.get()} (You)` },
      ]);

      return 'send';
    } catch (error) {
      console.error('Error send message:', error);
      return null;
    }
  }
}
