<script>
  import { NotificationList, Position } from '@whizzes/svelte-notifications';

  import '../app.css';
  import { fade } from 'svelte/transition';
</script>

<slot />
<NotificationList
  class="fixed top-1 right-6"
  position={Position.TopRight}
  let:notification
>
  <div
    in:fade
    out:fade
    class="bg-gray-100 rounded text-teal-900 px-4 py-3 shadow-md"
    role="alert"
  >
    <div class="flex">
      <div>
        <p class="font-bold">{notification.message}</p>
      </div>
    </div>
  </div>
</NotificationList>
