export interface Chat {
  id: string;
  title: string;
}

export interface ChatResponse {
  success: boolean;
  data?: Chat;
  error?: unknown;
}
