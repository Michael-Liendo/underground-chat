import fetch from '$lib/utils/fetch';

interface Chat {
  id: string;
  title: string;
}

export default class ChatService {
  static async createChat(title: string): Promise<Chat> {
    const response = await fetch('/chat/create', {
      method: 'POST',
      body: JSON.stringify({ title }),
    });

    const data: Chat = await response.json();

    return data;
  }
}
