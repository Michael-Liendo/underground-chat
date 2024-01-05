import { notifications } from '@whizzes/svelte-notifications';

// copy to clipboard action svelte
export const copyToClipboard = (text: string) => {
  notifications.notifySuccess('Copied to clipboard');

  const el = document.createElement('textarea');
  el.value = text;
  document.body.appendChild(el);
  el.select();
  document.execCommand('copy');
  document.body.removeChild(el);
};
