import fetch from '$lib/utils/fetch';
import { socket } from '../MessagesService';

import type { Chat, ChatResponse } from '$lib/types/Chat';

export default class ChatService {
  static async join(chatID: string) {
    socket.emit('join', chatID);
  }

  static async create(title: string): Promise<Chat | null> {
    try {
      const response = await fetch('/chat/create', {
        method: 'POST',
        body: JSON.stringify({ title }),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(
          `Chat creation failed with status ${response.status}: ${errorData.error}`,
        );
      }

      const data: ChatResponse = await response.json();

      if (!data.success || !data.data) {
        throw new Error(`Chat creation failed: ${data.error}`);
      }

      return data.data;
    } catch (error) {
      console.error('Error creating chat:', error);
      return null;
    }
  }
}
