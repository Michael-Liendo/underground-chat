import ChatService from './ChatService';
import MessagesService from './MessagesService';

export default class Service {
  static chat = ChatService;
  static message = MessagesService;
}
