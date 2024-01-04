<script lang="ts">
  /** @type {import('./$types').RouteParams} */
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { io } from 'socket.io-client';
  import TextField from '$lib/components/TextField.svelte';

  onMount(() => {
    const socket = io('http://127.0.0.1:3000/ws');

    setTimeout(() => {
      socket.emit('message', 'michael');
    }, 5000);
  });
</script>

<div class="flex h-screen">
  <aside class="w-64 bg-gray-800 text-white p-4 border-r border-r-slate-900">
    <h2 class="text-xl font-bold mb-4">Online Users</h2>
    <ul class="space-y-2">
      <li>John Doe</li>
      <li>Jane Smith</li>
      <li>Bob Johnson</li>
      <li>Alice Williams</li>
      <li>Charlie Brown</li>
    </ul>
  </aside>

  <main class="flex flex-col flex-1">
    <header
      class="flex items-center justify-between p-4 bg-gray-800 text-white"
    >
      <h1 class="text-2xl font-bold">CyberSec Chat ( Title )</h1>
      <div class="relative">
        <TextField
          name="message"
          class="bg-white text-black"
          placeholder="Search messages..."
          type="text"
        />
      </div>
    </header>

    <div
      dir="ltr"
      class="relative overflow-hidden flex-1 p-4 bg-gray-900 text-white"
      style="position: relative; --radix-scroll-area-corner-width: 0px; --radix-scroll-area-corner-height: 0px;"
    >
      <div
        data-radix-scroll-area-viewport=""
        class="h-full w-full rounded-[inherit]"
        style="overflow: hidden scroll;"
      >
        <div style="min-width: 100%; display: table;">
          <div class="space-y-4">
            <div>
              <h3 class="font-bold">
                John Doe <span class="text-sm text-gray-400">10:15 AM</span>
              </h3>
              <p>Hello, everyone!</p>
            </div>
            <div>
              <h3 class="font-bold">
                Jane Smith <span class="text-sm text-gray-400">10:16 AM</span>
              </h3>
              <p>Hi John, how are you?</p>
            </div>
          </div>
        </div>
      </div>
    </div>
    <footer class="p-4 bg-gray-800">
      <TextField
        name="message"
        class="bg-white text-black"
        placeholder="Type your message here..."
        type="text"
      />
    </footer>
  </main>
</div>

<style>
  [data-radix-scroll-area-viewport] {
    scrollbar-width: none;
    -ms-overflow-style: none;
    -webkit-overflow-scrolling: touch;
  }
  [data-radix-scroll-area-viewport]::-webkit-scrollbar {
    display: none;
  }
</style>
