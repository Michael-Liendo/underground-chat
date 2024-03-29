<script lang="ts">
  import { newForm } from '@whizzes/svelte-forms';
  import * as Yup from 'yup';

  import Button from '$lib/components/Button.svelte';
  import TextField from '$lib/components/TextField.svelte';
  import Service from '$lib/services';

  const { handleSubmit, values, errors } = newForm({
    initialValues: {
      id: '',
      name: '',
    },
    validationSchema: Yup.object({
      id: Yup.string().required().min(1, 'ID is too short'),
      name: Yup.string().required().min(1, 'Name is too short'),
    }),
    onSubmit: async (values) => {
      Service.user.create(values.name);

      window.location.pathname = `/chat/${values.id}`;
    },
  });
</script>

<svelte:head>
  <title>Join a Chat | Underground Chat</title>
  <meta
    name="description"
    content="Dive into a world of unfiltered conversations with Underground Chat. Connect anonymously, speak freely, and discover the power of authentic connection.
"
  />
  <meta
    name="keywords"
    content="anonymous chat, underground chat, private chat, secret chat, free chat, uncensored chat, real conversations, connect with strangers"
  />
  <!-- Schema.org markup for Google+ -->
  <meta itemprop="name" content="Join a Chat | Underground Chat" />
  <meta
    itemprop="description"
    content="Dive into a world of unfiltered conversations with Underground Chat. Connect anonymously, speak freely, and discover the power of authentic connection.
"
  />
  <!-- Open Graph data -->
  <meta property="og:title" content="Join a Chat | Underground Chat" />
  <meta property="og:type" content="article" />
  <meta property="og:url" content="https://michaelliendo.com/" />
  <meta
    property="og:description"
    content="Dive into a world of unfiltered conversations with Underground Chat. Connect anonymously, speak freely, and discover the power of authentic connection.
"
  />
</svelte:head>

<div class="flex justify-center items-center min-h-screen">
  <div
    class="w-full max-w-md p-6 m-auto bg-gray-800 rounded-xl shadow-md dark:bg-gray-800"
  >
    <h1 class="text-2xl font-semibold text-center text-white dark:text-white">
      Join a chat
    </h1>
    <form on:submit={handleSubmit} class="mt-4 space-y-4">
      <TextField
        name="id"
        label="Chat ID"
        placeholder="Enter chat id"
        error={$errors.id}
        bind:value={$values.id}
        autocomplete={'off'}
        required
      />

      <TextField
        name="name"
        label="Name"
        placeholder="Enter your name"
        error={$errors.name}
        bind:value={$values.name}
        required
      />

      <Button type="submit" fullWidth>Join</Button>
    </form>
    <div class="mt-4">
      <a class="text-indigo-500 hover:text-indigo-600 underline" href="/">
        Create a Chat
      </a>
    </div>
  </div>
</div>
