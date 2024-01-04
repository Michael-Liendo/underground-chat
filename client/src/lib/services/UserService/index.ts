export default class UserService {
  static async create(username: string) {
    if (typeof window !== 'undefined' && typeof window.localStorage !== 'undefined') {
      // Only use localStorage in the browser environment
      localStorage.setItem('username', username);
      console.log('User created successfully!');
    } else {
      // Handle cases where localStorage is not available (e.g., during SSR)
      console.log('localStorage not available, user data will not be persisted.');
    }
  }

  static delete() {
    if (typeof window !== 'undefined' && typeof window.localStorage !== 'undefined') {
      localStorage.removeItem('username');
      console.log('User deleted successfully!');
    } else {
      console.log('localStorage not available, user data cannot be deleted.');
    }
  }

  static get() {
    if (typeof window !== 'undefined' && typeof window.localStorage !== 'undefined') {
      return localStorage.getItem('username');
    } else {
      return null; // Return null if localStorage is not available
    }
  }
}