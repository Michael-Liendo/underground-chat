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
      title: Yup.string().required(),
      name: Yup.string().required(),
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
      <a class="text-indigo-500 hover:text-indigo-600 underline" href="/#">
        Join a Chat
      </a>
    </div>
  </div>
</div>
