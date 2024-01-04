<script lang="ts">
  /** @type {import('./$types').RouteParams} */
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { io } from 'socket.io-client';

  onMount(() => {
    const socket = io('http://127.0.0.1:3000/ws');

    setTimeout(() => {
      socket.emit('message', 'michael');
    }, 5000);
  });
</script>

<h1>{$page.params.id}</h1>
