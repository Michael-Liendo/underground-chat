<script lang="ts">
  import { newForm } from '@whizzes/svelte-forms';
  import * as Yup from 'yup';

  import Button from '$lib/components/Button.svelte';
  import TextField from '$lib/components/TextField.svelte';
  import Service from '$lib/services';

  const { handleSubmit, values, errors } = newForm({
    initialValues: {
      title: '',
      name: '',
    },
    validationSchema: Yup.object({
      title: Yup.string().required().min(1, 'Title is too short'),
      name: Yup.string().required().min(1, 'Name is too short'),
    }),
    onSubmit: async (values) => {
      const response = await Service.chat.create(values.title);
      const createUser = await Service.user.create(values.name);

      if (response?.id) {
        window.location.pathname = `/chat/${response.id}`;
      } else {
        console.log(response);
      }
    },
  });
</script>

<svelte:head>
  <title>Create a Chat | Underground Chat</title>

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
  <meta itemprop="name" content="Create a Chat | Underground Chat" />
  <meta
    itemprop="description"
    content="Dive into a world of unfiltered conversations with Underground Chat. Connect anonymously, speak freely, and discover the power of authentic connection.
  "
  />
  <!-- Open Graph data -->
  <meta property="og:title" content="Create a Chat | Underground Chat" />
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
      Create a chat
    </h1>
    <form on:submit={handleSubmit} class="mt-4 space-y-4">
      <TextField
        name="title"
        label="Title"
        placeholder="Enter chat title"
        error={$errors.title}
        bind:value={$values.title}
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

      <Button type="submit" fullWidth>Create</Button>
    </form>
    <div class="mt-4">
      <a class="text-indigo-500 hover:text-indigo-600 underline" href="/join">
        Join a Chat
      </a>
    </div>
  </div>
</div>
