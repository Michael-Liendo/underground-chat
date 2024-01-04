<script lang="ts">
  import { newForm } from '@whizzes/svelte-forms';

  /** @type {import('./$types').RouteParams} */
  import { page } from '$app/stores';
  import TextField from '$lib/components/TextField.svelte';
  import Service from '$lib/services';
  import { messagesStore } from '$lib/stores/messages';
  import { socket } from '$lib/services/MessagesService';
  import { onMount } from 'svelte';

  let listRef: HTMLElement | undefined;

  const { handleSubmit, values, errors } = newForm({
    initialValues: {
      message: '',
    },
    onSubmit: async (values, helper) => {
      Service.message.send(values.message, $page.params.id);

      helper.setFieldValue('message', '');
    },
  });

  $: {
    if (listRef) {
      listRef.scrollTo({
        behavior: 'smooth',
        top: 0,
      });
    }
  }

  onMount(() => {
    Service.chat.join($page.params.id);
  });
</script>

<div class="flex h-screen">
  <aside class="w-64 bg-gray-800 text-white p-4 border-r border-r-slate-900">
    <h2 class="text-xl font-bold mb-4">Online Users</h2>
    <ul class="space-y-2">
      <li>John Doe</li>
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
          <div bind:this={listRef} class="space-y-4">
            {#each $messagesStore as message}
              <div>
                <h3 class="font-bold">
                  {message.username}
                  <span class="text-sm text-gray-400">10:16 AM</span>
                </h3>
                <p>{message.content}</p>
              </div>
            {/each}
          </div>
        </div>
      </div>
    </div>
    <footer class="p-4 bg-gray-800">
      <form on:submit={handleSubmit}>
        <TextField
          name="message"
          class="bg-white text-black"
          placeholder="Type your message here..."
          type="text"
          bind:value={$values.message}
          error={$errors.message}
        />
        <input type="submit" hidden />
      </form>
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
