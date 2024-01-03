<script lang="ts">
  import classNames from 'classnames';

  export let type: 'button' | 'submit' = 'button';
  export let fullWidth = false;
  export let disabled = false;
  export let isLoading = false;

  let customClassNames = '';
  export { customClassNames as class };

  let className = classNames(
    customClassNames,
    'inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 h-10 px-4 py-2 bg-indigo-500 hover:bg-indigo-600 text-white',
    {
      'w-full': fullWidth,
      'disabled:opacity-50 disabled:pointer-events-none  !text-gray-400':
        isLoading || disabled,
      '!cursor-not-allowed': disabled,
      '!cursor-progress': isLoading,
    }
  );
</script>

<button {type} class={className} {disabled} on:click>
  <slot />
</button>
