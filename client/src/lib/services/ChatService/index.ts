import fetch from '$lib/utils/fetch';
import { socket } from '../MessagesService';
import UserService from '../UserService';

import type { Chat, ChatResponse } from '$lib/types/Chat';
import { activeUsersStore } from '$lib/stores/users';

export default class ChatService {
  static async join(room: string) {
    let username = UserService.get()
    socket.emit('join', {room, username});

    activeUsersStore.update((users) => [...users, {username: `${username} (You)`}]);
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
