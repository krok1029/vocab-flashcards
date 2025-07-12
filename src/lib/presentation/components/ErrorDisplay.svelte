<script lang="ts">
  import * as Alert from '$lib/presentation/components/ui/alert/index.js';

  export let error: string;
  export let onRetry: (() => void) | null = null;
</script>

<Alert.Root class="border-red-200 bg-red-50">
  <Alert.Title class="text-red-800">錯誤</Alert.Title>
  <Alert.Description class="text-red-700">
    {error}
    {#if onRetry}
      <button 
        on:click={onRetry}
        class="ml-2 text-red-600 underline hover:text-red-800"
      >
        重試
      </button>
    {/if}
  </Alert.Description>
</Alert.Root>
