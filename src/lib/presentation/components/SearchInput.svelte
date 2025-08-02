<script lang="ts">
  import Button from '$lib/presentation/components/ui/button/button.svelte';
  import Input from '$lib/presentation/components/ui/input/input.svelte';

  interface Props {
    query?: string;
    loading?: boolean;
    canSave?: boolean;
    onsearch?: (query: string) => void;
    onsave?: () => void;
  }

  let { query = '', loading = false, canSave = false, onsearch, onsave }: Props = $props();

  // Internal state for the input field
  let inputValue = $state(query);

  // Update internal state when external query changes
  $effect(() => {
    inputValue = query;
  });

  function handleSearch() {
    console.log('Searching for:', inputValue);
    onsearch?.(inputValue);
  }

  function handleSave() {
    console.log('Saving word card');
    onsave?.();
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
    bind:value={inputValue}
    onkeydown={handleKeydown}
    disabled={loading}
  />
  <Button 
    class="bg-blue-500 text-white px-4 py-1 rounded hover:bg-blue-600 disabled:opacity-50" 
    onclick={handleSearch}
    disabled={loading}
  >
    {loading ? '載入中...' : '查詢'}
  </Button>
  <Button
    class="px-4 py-1 rounded bg-green-600 text-white hover:bg-green-700 disabled:opacity-50"
    onclick={handleSave}
    disabled={!canSave || loading}
  >
    加入單字卡
  </Button>
</div>
