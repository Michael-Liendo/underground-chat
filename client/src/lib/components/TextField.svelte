<script lang="ts">
  import classNames from 'classnames';

  export let type: 'text' | 'number' | 'email' | 'password' | 'date' = 'text';
  export let name: string;
  export let id = name;
  export let error: string | null = null;
  export let value: string | number | Date | null = null;
  export let label: string | null = null;
  export let placeholder: string | undefined = undefined;
  export let required = false;
  export let autocomplete: 'true' | 'false' | undefined = undefined;
  let customClassNames = '';
  export { customClassNames as class };

  let className = classNames(
    customClassNames,
    'flex h-10 w-full rounded-md border text-black border-input px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 bg-gray-700 placeholder-gray-500'
  );
  const handleInput = (event: Event): void => {
    const target = event.target as HTMLInputElement;
    value = type.match(/^(number|range)$/) ? +target.value : target.value;
  };
</script>

<div>
  {#if label}
    <label
      for={name}
      class:text-red-600={!!error}
      class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 text-white mb-2"
      >{label}</label
    >
  {/if}

  <input
    {name}
    {id}
    {placeholder}
    {required}
    {autocomplete}
    {type}
    {value}
    class:border-red-600={!!error}
    class={className}
    on:change
    on:blur
    on:input={handleInput}
  />
  {#if error}
    <p class:opacity-0={!error} class="text-sm pt-1 text-red-600">{error}</p>
  {/if}
</div>
