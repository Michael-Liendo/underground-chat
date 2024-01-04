import { io } from 'socket.io-client';

interface Chat {
  id: string;
  title: string;
}

interface ChatResponse {
  success: boolean;
  data?: Chat;
  error?: unknown;
}

const socket = io(`${import.meta.env.VITE_API_URL}/ws`);

export default class MessagesService {
  static async send(content: string, chatID: string): Promise<string | null> {
    try {
      socket.emit('chat message', content, (response: unknown) => {
        console.log(response);
      });

      return 'send';
    } catch (error) {
      console.error('Error send message:', error);
      return null;
    }
  }
}
