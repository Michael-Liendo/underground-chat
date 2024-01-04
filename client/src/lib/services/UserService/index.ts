export default class UserService {
  static async create(username: string) {
    if (typeof window !== 'undefined' && typeof window.localStorage !== 'undefined') {
      localStorage.setItem('username', username);
    } else {
      console.log('localStorage not available, user data will not be persisted.');
    }
  }

  static delete() {
    if (typeof window !== 'undefined' && typeof window.localStorage !== 'undefined') {
      localStorage.removeItem('username');
    } else {
      console.log('localStorage not available, user data cannot be deleted.');
    }
  }

  static get() {
    if (typeof window !== 'undefined' && typeof window.localStorage !== 'undefined') {
      return localStorage.getItem('username') ?? 'Anonymous';
    } else {
      return 'Anonymous'; // Return null if localStorage is not available
    }
  }
}