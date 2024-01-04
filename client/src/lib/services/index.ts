import ChatService from './ChatService';
import MessagesService from './MessagesService';
import UserService from './UserService';

export default class Service {
  static chat = ChatService;
  static message = MessagesService;
  static user = UserService;
}
