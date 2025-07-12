<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Button from '$lib/presentation/components/ui/button/button.svelte';
  import Input from '$lib/presentation/components/ui/input/input.svelte';

  export let query = '';
  export let loading = false;
  export let canSave = false;

  const dispatch = createEventDispatcher<{
    search: string;
    save: void;
  }>();

  function handleSearch() {
    console.log('Searching for:', query);
    dispatch('search', query);
  }

  function handleSave() {
    console.log('Saving word card');
    dispatch('save');
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      handleSearch();
    }
  }
</script>

<div class="flex gap-2">
  <Input
    class="flex-1"
    placeholder="輸入單字..."
    bind:value={query}
    on:keydown={handleKeydown}
    {disabled}={loading}
  />
  <Button 
    class="bg-blue-500 text-white px-4 py-1 rounded hover:bg-blue-600 disabled:opacity-50" 
    on:click={handleSearch}
    disabled={loading}
  >
    {loading ? '載入中...' : '查詢'}
  </Button>
  <Button
    class="px-4 py-1 rounded bg-green-600 text-white hover:bg-green-700 disabled:opacity-50"
    on:click={handleSave}
    disabled={!canSave || loading}
  >
    加入單字卡
  </Button>
</div>
