<script lang="ts">
  import { newForm } from '@whizzes/svelte-forms';
  import { onMount } from 'svelte';
  import * as Yup from 'yup';

  /** @type {import('./$types').RouteParams} */
  import { page } from '$app/stores';
  import TextField from '$lib/components/TextField.svelte';
  import Service from '$lib/services';
  import { messagesStore } from '$lib/stores/messages';
  import getHour from '$lib/utils/DateToHours';
  import ScrollTo from '$lib/actions/ScrollTo';
  import { activeUsersStore } from '$lib/stores/users';

  let listRef: HTMLElement;

  const { handleSubmit, values, errors } = newForm({
    initialValues: {
      message: '',
    },
    validationSchema: Yup.object({
      message: Yup.string().required().min(1, 'Message is too short'),
    }),
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
    messagesStore.subscribe(() => {
      requestAnimationFrame(() => {
        ScrollTo('down', listRef);
      });
    });
  });

  onMount(() => {
    Service.chat.join($page.params.id);
  });
</script>

<svelte:head>
  <title>Underground Chat | {$page.params.id}</title>
</svelte:head>

<div class="flex h-screen">
  <aside class="w-full md:w-64 bg-gray-800 text-white p-4 md:block hidden">
    <h2 class="text-xl font-bold mb-4">Last Users Logged</h2>
    <ul class="space-y-2">
      {#each $activeUsersStore as user}
        <li>{user.username}</li>
      {/each}
    </ul>
  </aside>
  <main class="flex flex-col flex-1 w-full">
    <header
      class="flex items-center justify-between p-4 bg-gray-800 text-white"
    >
      <h1 class="text-lg md:text-2xl font-bold truncate">{$page.params.id}</h1>
      <div class="relative w-[100%] hidden">
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
        class="h-full w-full rounded-[inherit] space-y-4"
        style="overflow: hidden scroll;"
        bind:this={listRef}
      >
        {#each $messagesStore as message}
          <div>
            <h3 class="font-bold">
              {message.username}
              <span class="text-sm text-gray-400"
                >{getHour(message.created_at)}</span
              >
            </h3>
            <p>{message.content}</p>
          </div>
        {/each}
      </div>
    </div>
    <footer class="p-4 bg-gray-800">
      <form on:submit={handleSubmit}>
        <TextField
          name="message"
          class="bg-white !text-black"
          placeholder="Type your message here..."
          type="text"
          autocomplete="off"
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
